# conflu-mcp-rs

A Model Context Protocol (MCP) server for Confluence, written in Rust.

## Features

This MCP server provides simple tools for interacting with Confluence:

- **Read content**: Get page details
- **Create content**: Create new pages with title and body content
- **Edit content**: Update existing pages with new content

## Prerequisites

- Rust 1.75 or higher
- Confluence Cloud account
- Confluence API token

## Installation

```bash
cargo build --release
```

## Configuration

Set the following environment variables:

```bash
export CONFLUENCE_BASE_URL="https://your-domain.atlassian.net"
export CONFLUENCE_EMAIL="your-email@example.com"
export CONFLUENCE_API_TOKEN="your-api-token"
```

### Getting a Confluence API Token

1. Go to https://id.atlassian.com/manage-profile/security/api-tokens
2. Click "Create API token"
3. Give it a name and copy the token

### Claude Code Configuration

Add this to your Claude Code MCP settings file (`~/.claude/settings.json`):

```json
{
  "mcpServers": {
    "confluence": {
      "command": "/path/to/conflu-mcp-rs/target/release/conflu-mcp-rs",
      "env": {
        "CONFLUENCE_BASE_URL": "https://your-domain.atlassian.net",
        "CONFLUENCE_EMAIL": "your-email@example.com",
        "CONFLUENCE_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

Replace `/path/to/conflu-mcp-rs` with the actual path to your project directory, and update the environment variables with your Confluence credentials.

## Usage

Run the server:

```bash
cargo run
```

Or use the compiled binary:

```bash
./target/release/conflu-mcp-rs
```

## Available Tools

### get_page

Get a Confluence page by its ID.

**Parameters:**
- `page_id` (string): The page ID to retrieve



### create_page

Create a new Confluence page.

**Parameters:**
- `space_id` (string): The space ID where the page will be created
- `title` (string): The title of the new page
- `body` (string): The body content in Confluence storage format (HTML-like format)
- `parent_id` (optional string): Optional parent page ID

### update_page

Update an existing Confluence page.

**Parameters:**
- `page_id` (string): The page ID to update
- `title` (string): The new title for the page
- `body` (string): The new body content in Confluence storage format
- `version_number` (number): The current version number of the page (required for optimistic locking)
- `version_message` (optional string): Optional version message describing the changes

## Confluence Storage Format

The body content should be in Confluence storage format, which is an HTML-like format:

```html
<p>This is a paragraph.</p>
<h2>This is a heading</h2>
<ul>
  <li>List item 1</li>
  <li>List item 2</li>
</ul>
```

## License

MIT

## Author

Katsuhiro Honda
