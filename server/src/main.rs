#[macro_use]
extern crate rocket;

use clap::Parser;
use diesel::PgConnection;
use figment::{
    providers::{Env, Format, Toml},
    Figment, Profile,
};
use rocket_sync_db_pools::database;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, parse(from_os_str))]
    config: Option<std::path::PathBuf>,
}

mod account;
mod balance;
mod error;
mod expenditure;
mod repayment;
mod user;

#[database("main")]
struct MainDbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    let args = Cli::parse();
    let config_provider = match args.config {
        Some(path) => Figment::from(rocket::Config::default())
            .select(Profile::from_env_or(
                "ROCKET_PROFILE",
                rocket::Config::DEFAULT_PROFILE,
            ))
            .merge(Toml::file(path).nested())
            .merge(Env::prefixed("ROCKET_").ignore(&["PROFILE"]).global()),
        None => rocket::Config::figment(),
    };
    rocket::build()
        .configure(config_provider)
        .attach(MainDbConn::fairing())
        .mount(
            "/",
            routes![
                account::post_account,
                account::get_account,
                expenditure::post_expenditure,
                expenditure::get_expenditures,
                expenditure::del_expenditure,
                expenditure::put_expenditure,
                repayment::post_repayment,
                repayment::get_repayments,
                repayment::del_repayment,
                repayment::put_repayment,
                user::post_user,
                user::put_user,
                user::get_users,
                balance::get_balance,
            ],
        )
}
