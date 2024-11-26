use serde::Deserialize;
use ureq::Agent;

#[derive(Deserialize)]
struct Word {
  #[serde(rename = "word")]
  name: String,
  url: String,
}

pub fn search_word(word: String) {
  println!("Searching for the word: {}\n", word.to_uppercase());
  let client = ureq::builder()
    .redirects(0)
    .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
    .build();

  let res = client
    .get(format!("https://dictionary.cambridge.org/dictionary/english/{word}").as_str())
    .call();

  let res = match res {
    Ok(res) => res,
    Err(ureq::Error::Status(code, res)) => match code {
      500 => {
        eprintln!("Server failed to respond back");
        return;
      }
      501..599 => {
        eprintln!("Server returned: {code}");
        return;
      }
      _ => res,
    },
    Err(err) => {
      eprintln!("IO Error occur: {err}");
      return;
    }
  };

  if res.status() == 302 {
    let res = client
      .get(format!("https://dictionary.cambridge.org/spellcheck/english/?q={word}").as_str())
      .call()
      .expect("failed to send request for spellchecker");
    let html_string = res
      .into_string()
      .expect("failed to convert response into string");
    let doc = tl::parse(html_string.as_str(), tl::ParserOptions::default())
      .expect("failed to parse html response");

    eprintln!("No results found for \"{word}\"");
    println!("Similar spelling or pronunciations:");
    let li_node = doc.query_selector("li.lbt.lp-5.lpl-20").unwrap();
    for li in li_node {
      let li_elm = li.get(doc.parser()).unwrap();
      let li_tag = li_elm.as_tag().unwrap();
      let text = li_tag.inner_text(doc.parser());
      println!("• {}", text.trim_ascii());
    }
    return;
  }

  let body = res
    .into_string()
    .expect("failed to convert the response into string (type)");
  let dom =
    tl::parse(&body, tl::ParserOptions::default()).expect("failed to parse the HTML response");

  let desc_div_node = dom
    .query_selector("div.def.ddef_d.db")
    .unwrap()
    .next()
    .unwrap();
  let desc_div_tag = desc_div_node.get(dom.parser()).unwrap().as_tag().unwrap();
  let desc = desc_div_tag.inner_text(dom.parser());
  let mut desc_chars = desc.trim().chars();
  // Remove the last character ":"
  desc_chars.next_back();
  println!("{}", desc_chars.as_str());

  println!("\nExamples:");
  let span_nodes = dom.query_selector("span.eg.deg").unwrap();
  for span_elm in span_nodes {
    let span = span_elm.get(dom.parser()).unwrap();
    let span_tag = span.as_tag().unwrap();
    println!("• {}", span_tag.inner_text(dom.parser()))
  }

  println!("\nSource: https://dictionary.cambridge.org/dictionary/english/{word}")
}

pub fn find_word(word: String) {
  let client = ureq::builder()
    .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
    .build();

  println!("Searching for words similar to: {}", word.to_uppercase());

  let res = client
    .get(
      format!("https://dictionary.cambridge.org/autocomplete/amp?dataset=english&q={word}")
        .as_str(),
    )
    .call();

  let res = match res {
    Ok(res) => res,
    Err(ureq::Error::Status(code, res)) => match code {
      500 => {
        eprintln!("Server failed to respond back");
        return;
      }
      501..599 => {
        eprintln!("Server returned: {code}");
        return;
      }
      _ => res,
    },
    Err(err) => {
      eprintln!("IO Error occur: {err}");
      return;
    }
  };

  let json = res
    .into_string()
    .expect("failed to convert the response into string (type)");
  let words: Box<[Word]> =
    serde_json::from_str(json.as_str()).expect("failed to deserialize json response");
  if words.is_empty() {
    println!("No results found");
    return;
  }

  let res = client
    .get(format!("https://dictionary.cambridge.org{}", words[0].url).as_str())
    .call()
    .expect("failed to make a request");

  let res_html = res
    .into_string()
    .expect("failed to convert response into string string");
  let doc = tl::parse(res_html.as_str(), tl::ParserOptions::default()).unwrap();
  let div_node_hanlder = doc
    .query_selector("div.def.ddef_d.db")
    .expect("failed to find the div element with this query-selector: 'div.def.ddef_d.db'")
    .next()
    .unwrap();
  let div_tag = div_node_hanlder
    .get(doc.parser())
    .expect("failed to get the Node for the query-selector 'div.def.ddef_d.db'")
    .as_tag()
    .expect("failed to get the HTML Tag for the query-selector 'div.def.ddef_d.db'");

  println!(
    "\n{} means: {}",
    words[0].name.to_uppercase(),
    div_tag.inner_text(doc.parser())
  );

  println!("Found {} similar words:", words.len() - 1);

  for word in words.iter().skip(1) {
    println!("• {}", word.name)
  }
}

