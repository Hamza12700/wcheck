use serde::Deserialize;

#[derive(Deserialize)]
struct Word {
  #[serde(rename = "word")]
  name: String,
  // url: String,
}

pub fn search_word(word: String) {
  println!("Searching for the word: {}\n", word.to_uppercase());

  let res =
    ureq::get(format!("https://dictionary.cambridge.org/dictionary/english/{word}").as_str())
    .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
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

  let body = res
    .into_string()
    .expect("failed to convert the response into string (type)");
  let dom =
    tl::parse(&body, tl::ParserOptions::default()).expect("failed to parse the HTML response");

  let meta_node = dom
    .query_selector("meta[name=\"description\"]")
    .unwrap()
    .next()
    .unwrap();
  let meta_node = meta_node.get(dom.parser()).unwrap();
  let meta_elm = meta_node.as_tag().unwrap();
  let desc_bytes = meta_elm.attributes().get("content").unwrap().unwrap();
  let desc_text = desc_bytes.try_as_utf8_str().unwrap();
  let mut desc_split: Vec<_> = desc_text.split(".").collect();

  // Remove the Text at the beginner of the string
  desc_split.remove(0);
  desc_split.remove(desc_split.len() - 1);
  desc_split.remove(desc_split.len() - 1);

  for text in desc_split.iter() {
    let parts: Box<[_]> = text.split(":").collect();
    let mut desc = parts[0];
    desc = desc.trim_end_matches("&hellip;");
    println!("• {}", desc.trim_ascii());
  }

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
  println!("Searching for words similar to: {word}");

  let res =
    ureq::get(format!("https://dictionary.cambridge.org/autocomplete/amp?dataset=english&q={word}").as_str())
    .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
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
  let words: Vec<Word> =
    serde_json::from_str(json.as_str()).expect("failed to deserialize json response");
  if words.is_empty() {
    println!("No results found");
    return;
  }
  println!("Found {} similar words", words.len());

  for word in words {
    println!("• {}", word.name)
  }
}
