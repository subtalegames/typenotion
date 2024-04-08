use clap::Parser;
use codegen::Scope;
use reqwest::blocking::Client;
use serde::Deserialize;

/// The arguments for the program, parsed using clap.
#[derive(Parser, Debug)]
struct Args {
    /// The Notion database ID, used to populate the values of the generated
    /// enum.
    database_id: String,
    /// The name of the enum to generate. If not provided, the name of the
    /// database will be used.
    #[arg(short, long)]
    name: Option<String>,
    /// Derive traits to add to the generated enum.
    #[arg(long)]
    derive: Vec<String>,
    /// The output file to write the generated code to. If not provided, the
    /// enum name will be used (lowercased and suffixed with ".rs").
    #[arg(short, long)]
    output_file: Option<String>,
    /// Whether to generate an automatic Display implementation for the enum.
    #[arg(long)]
    generate_display: bool,
    /// Whether to write the generated code to stdout instead of a file.
    #[arg(long)]
    stdout: bool,
    /// Whether to generate enum documentation from the Notion records.
    #[arg(long)]
    generate_docs: bool,
}

/// The response from the Notion API when querying a database's records.
#[derive(Deserialize, Debug)]
struct Response {
    /// The records returned by the query.
    results: Vec<Record>,
}

/// The response from the Notion API when querying a page's children blocks.
#[derive(Deserialize, Debug)]
struct BlockResponse {
    /// The blocks returned by the query.
    results: Vec<BlockRecord>,
}

/// The response from the Notion API when querying a database's metadata.
#[derive(Deserialize, Debug)]
struct DatabaseResponse {
    /// The title of the database.
    title: Vec<TitleContent>,
}

/// A block record from the Notion API.
#[derive(Deserialize, Debug)]
struct BlockRecord {
    /// The paragraph block, if present.
    paragraph: Option<Paragraph>,
}

/// A paragraph block from the Notion API.
#[derive(Deserialize, Debug)]
struct Paragraph {
    /// The rich text content of the paragraph.
    rich_text: Vec<RichText>,
}

/// Rich text content from the Notion API.
#[derive(Deserialize, Debug)]
struct RichText {
    /// The text content of the rich text.
    text: TextContent,
}

/// A record from the Notion API.
#[derive(Deserialize, Debug)]
struct Record {
    /// The ID of the record.
    id: String,
    /// The properties of the record.
    properties: Properties,
    /// The URL of the record.
    url: String,
}

/// The properties of a record from the Notion API.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Properties {
    Name: Name,
}

#[derive(Deserialize, Debug)]
struct Name {
    title: Vec<TitleContent>,
}

/// A title content from the Notion API.
#[derive(Deserialize, Debug)]
struct TitleContent {
    /// The text content of the title.
    text: TextContent,
}

/// The text content of a block from the Notion API.
#[derive(Deserialize, Debug)]
struct TextContent {
    /// The content of the text.
    content: String,
}

struct Achievement {
    id: String,
    name: String,
    enum_name: String,
    description: Vec<String>,
    url: String,
}

fn main() {
    // Read Notion API key from environment variable
    let notion_api_key = std::env::var("NOTION_API_KEY")
        .expect("Please set the NOTION_API_KEY environment variable");

    let args = Args::parse();

    // Setup default headers for Notion's API
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", notion_api_key))
            .expect("Failed to create header"),
    );
    headers.insert(
        "Notion-Version",
        reqwest::header::HeaderValue::from_static("2022-06-28"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build client");

    // If no name is provided, query notion to get the database name
    let enum_name = if args.name.is_none() {
        let database_response = client
            .get(format!(
                "https://api.notion.com/v1/databases/{}",
                args.database_id
            ))
            .send()
            .expect("Failed to send request")
            .json::<DatabaseResponse>()
            .expect("Failed to parse response");

        database_response.title[0].text.content.clone()
    } else {
        args.name.clone().unwrap()
    };

    // Query the Notion API to get the database
    let response = client
        .post(format!(
            "https://api.notion.com/v1/databases/{}/query",
            args.database_id,
        ))
        .send()
        .expect("Failed to send request")
        .json::<Response>()
        .expect("Failed to parse response");

    // Parse response into vec of achievement structs
    let mut achievements = response
        .results
        .iter()
        .map(|record| Achievement {
            id: record.id.clone(),
            name: record.properties.Name.title[0].text.content.clone(),
            enum_name: record.properties.Name.title[0]
                .text
                .content
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<Vec<String>>()
                .join(""),
            url: record.url.clone(),
            description: Vec::new(),
        })
        .collect::<Vec<Achievement>>();

    let mut scope = Scope::new();

    // Parse enun name from environment variable
    let generated_enum = scope.new_enum(enum_name.clone().replace(' ', ""));

    // Add derives to the enum if passed as a comma separated list in the DERIVES
    // environment variable
    for derive in args.derive.iter() {
        generated_enum.derive(derive);
    }

    // For each achievement, query the Notion API to get the full record's block
    // children paragraphs
    if args.generate_docs {
        for achievement in achievements.iter_mut() {
            let response = client
                .get(format!(
                    "https://api.notion.com/v1/blocks/{}/children",
                    achievement.id
                ))
                .send()
                .expect("Failed to send request")
                .json::<BlockResponse>()
                .expect("Failed to parse response");

            // Parse response into vec of strings
            achievement.description = response
                .results
                .iter()
                .filter_map(|block_record| block_record.paragraph.as_ref())
                .flat_map(|paragraph| paragraph.rich_text.iter())
                .map(|rich_text| rich_text.text.content.clone())
                .collect();
        }
    }

    for achievement in achievements.iter() {
        let variant = generated_enum.new_variant(&achievement.enum_name);

        if !achievement.description.is_empty() {
            variant.annotation(format!(
                "/// ## {}\n///\n/// {}\n///\n/// [Link to Notion record]({})",
                achievement.name,
                achievement.description.join("\n///\n/// "),
                achievement.url,
            ));
        }
    }

    if args.generate_display {
        let trait_impl = scope
            .new_impl(&enum_name.replace(' ', ""))
            .impl_trait("std::fmt::Display");

        let fmt_fn = trait_impl.new_fn("fmt");
        fmt_fn.arg_ref_self();
        fmt_fn.arg("f", "&mut std::fmt::Formatter");
        fmt_fn.ret("std::fmt::Result");
        fmt_fn.line(format!("use {}::*;", enum_name.replace(' ', "")));
        fmt_fn.line("match self {");
        for achievement in achievements.iter() {
            fmt_fn.line(format!(
                "    {} => write!(f, \"{}\"),",
                achievement.enum_name, achievement.name
            ));
        }
        fmt_fn.line("}");
    }

    let file_contents = format!(
        "// Generated at {}\n\n{}\n",
        chrono::Utc::now().to_rfc3339(),
        scope.to_string()
    );

    // Write to stdout if the --stdout flag is passed, otherwise write to output
    // file (defaulting to enum name + ".rs")
    if args.stdout {
        println!("{}", file_contents);
    } else {
        let output_file = args
            .output_file
            .unwrap_or_else(|| format!("{}.rs", enum_name.to_lowercase().replace(' ', "_")));
        std::fs::write(output_file, file_contents).expect("Failed to write to file");
    }
}
