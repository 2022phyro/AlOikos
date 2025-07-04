use aloikos_web::authentication::jwt::{Blacklist, JwtAccessToken, JwtRefreshToken, Token};
use aloikos_web::config::CONFIG;
use aloikos_web::db::connect_db;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Aloikos Web Application");

    // Display configuration
    println!("\n⚙️  Database Configuration:");
    println!("   DB URI: {}", CONFIG.db_uri);
    println!("   DB Name: {}", CONFIG.db_name);

    // Test database connection
    println!("\n🔗 Testing database connection...");
    match connect_db().await {
        Ok(_) => println!("✅ Database connection successful!"),
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            eprintln!("\n💡 Suggestions:");
            eprintln!("   1. Check if DATABASE_URL is set in your .env file");
            eprintln!("   2. For SQLite: Make sure the directory exists");
            eprintln!("   3. For PostgreSQL/MySQL: Check server is running");
            return;
        }
    }

    println!("\n🎉 Application started successfully!");
}
