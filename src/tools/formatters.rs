use crate::confluence::{Page, PageListResponse};

pub fn format_page(page: &Page) -> String {
    let space_id = page.space_id.as_deref().unwrap_or("Unknown");
    let parent_id = page.parent_id.as_deref().unwrap_or("None");
    let created_at = page.created_at.as_deref().unwrap_or("Unknown");
    let version_number = page.version.as_ref().map(|v| v.number).unwrap_or(0);

    let web_ui_link = page
        .links
        .as_ref()
        .and_then(|l| l.web_ui.as_deref())
        .unwrap_or("Not available");

    let body_preview = page
        .body
        .as_ref()
        .and_then(|b| b.storage.as_ref())
        .map(|b| {
            let preview = b.value.chars().take(200).collect::<String>();
            if b.value.len() > 200 {
                format!("{}...", preview)
            } else {
                preview
            }
        })
        .unwrap_or_else(|| "No content".to_string());

    format!(
        r#"# {} (ID: {})

**Status:** {}
**Space ID:** {}
**Parent ID:** {}
**Version:** {}
**Created:** {}
**Web UI:** {}

### Content Preview
{}
"#,
        page.title,
        page.id,
        page.status,
        space_id,
        parent_id,
        version_number,
        created_at,
        web_ui_link,
        body_preview
    )
}

pub fn format_page_list(response: &PageListResponse, space_id: &str) -> String {
    if response.results.is_empty() {
        return format!("No pages found in space {}", space_id);
    }

    let mut output = format!(
        "Found {} page(s) in space {}:\n\n",
        response.results.len(),
        space_id
    );

    for page in &response.results {
        let version_number = page.version.as_ref().map(|v| v.number).unwrap_or(0);
        output.push_str(&format!(
            "- **{}** (ID: {})\n  Status: {} | Version: {}\n\n",
            page.title, page.id, page.status, version_number
        ));
    }

    if response.links.as_ref().and_then(|l| l.next.as_ref()).is_some() {
        output.push_str("(More pages available - use pagination to retrieve)\n");
    }

    output
}

pub fn format_page_created(page: &Page) -> String {
    let web_ui_link = page
        .links
        .as_ref()
        .and_then(|l| l.web_ui.as_deref())
        .unwrap_or("Not available");

    format!(
        r#"Page created successfully!

**Title:** {}
**ID:** {}
**Space ID:** {}
**Status:** {}
**Web UI:** {}
"#,
        page.title,
        page.id,
        page.space_id.as_deref().unwrap_or("Unknown"),
        page.status,
        web_ui_link
    )
}

pub fn format_page_updated(page: &Page) -> String {
    let version_number = page.version.as_ref().map(|v| v.number).unwrap_or(0);
    let web_ui_link = page
        .links
        .as_ref()
        .and_then(|l| l.web_ui.as_deref())
        .unwrap_or("Not available");

    format!(
        r#"Page updated successfully!

**Title:** {}
**ID:** {}
**New Version:** {}
**Status:** {}
**Web UI:** {}
"#,
        page.title, page.id, version_number, page.status, web_ui_link
    )
}
