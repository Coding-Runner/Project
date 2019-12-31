extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main(){
    println!("Welcome! The data we are going to get is: ");
    scrape_team_data("https://www.scoreboard.com/game/dallas-mavericks-golden-state-warriors-2019-2020/CYWskeOu/#game-summary|game-statistics;0|lineups;1");
}

fn scrape_team_data(url:&str){

    let mut req = reqwest::get(url).unwrap();
    assert!(req.status().is_success());
    let doc_body = Html::parse_document(&req.text().unwrap());

    let team = Selector::parse(".scoreboard").unwrap();

    for team in doc_body.select(&team){
        let teams = team.text().collect::<Vec<_>>();
        println!("{}", teams[0]);
    }

}