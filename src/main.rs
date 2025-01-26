fn main() {
    let s1 = String::from("Les gens sont incroyables, mais pas toujours dans le bon sens...");
    let s2 = String::from("C'est fou comme Yafoy est court et percutant!");

    let result = longest(s1.as_str(), s2.as_str());
    println!("📏 La phrase la plus longue est : \"{}\"", result);

    let comment = match result {
        res if res == s1 => "🤔 On dirait que tu es du genre à en dire beaucoup pour si peu !",
        res if res == s2 => "💡 Bref, efficace, concis. T'es une vraie punchline ambulante !",
        _ => "😮 Wow, comment on en est arrivé là ?",
    };
    println!("{}", comment);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
