use anyhow::Result;

use ghactions::ToolCache;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Tool Cache Example:\n");

    // Use the local in repository tool cache (testing)
    let tool_cache = ToolCache::from("./examples/toolcache");

    // Find a tool in the cache (node 12.7.0)
    let path = tool_cache
        .find("node", "12.7.0")
        .await
        .expect("Failed to find tool in cache");
    println!("Node (exact) :: {}", path);

    // Find a tool in the cache (node 12.x)
    let path = tool_cache
        .find("node", "12.x")
        .await
        .expect("Failed to find tool in cache");
    println!("Node (fuzzy version) :: {}", path);

    // Find a tool in the cache (node 12.7.0, x64)
    let path = tool_cache
        .find_with_arch("node", "12.7.0", "x64")
        .await
        .expect("Failed to find tool in cache");
    println!("Node (exact, x64) :: {}", path);

    // Find all versions of the tool in the cache (node)
    let versions = tool_cache
        .find_all_version("node")
        .await
        .expect("Failed to find all versions of tool in cache");

    println!("\nNode Versions ::");
    versions.iter().for_each(|v| println!("- Node :: {}", v));

    let node = tool_cache.find("node", "12.7.0").await?.join("node");
    println!("\nNode Path :: {:?}", node);

    // What happens when the tool is not found in the cache?
    match tool_cache.find("non-existent-tool", "1.0.0").await {
        Ok(path) => {
            // If this is reached, something went wrong
            panic!("Found non-existent tool :: {:?}", path);
        }
        Err(e) => {
            println!("\nNon-existent tool :: {:?}", e);
        }
    };

    Ok(())
}
