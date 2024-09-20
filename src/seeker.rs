pub mod seeker{

  use scraper::{html, Html, Selector};

  pub struct ThreadReply{
    pub reply: String,
    pub subject: String,
    pub image_response: String,
    pub is_op: bool,
    // pub datetime: String // maybe work with dates later
  }

  pub struct Thread{
    pub url: String,
    pub title: String,
    pub image_href: String, // i'll get some image rendering here, not on the terminal ofc
    pub reply: Vec<ThreadReply>

  }

  fn parse_html_thread(content: scraper::html::Html) -> Thread{
    let url_selector = Selector::parse("a").unwrap();
    let image_selector = Selector::parse(".thumb").unwrap();
    let a_link: Vec<Option<&str>> = content.select(&url_selector).map(|x| x.attr("href")).collect();
    let img_link: Vec<Option<&str>> = content.select(&image_selector).map(|x| x.attr("src")).collect();
    let title = content.select(&Selector::parse(".teaser").unwrap()).flat_map(|x| x.text()).collect::<String>();
    let url = a_link[0].unwrap_or("value not found");
    let image_href = img_link[0].unwrap_or("image not found");
    return Thread {url: url.to_string(), image_href: image_href.to_string(), reply: vec![], title};
  }

  pub fn parse_html_reply(content: &str, is_op: Option<bool>) -> ThreadReply{
    let document = scraper::Html::parse_document(content);
    let title_selector = Selector::parse(".postInfo .desktop > .subject").unwrap();
    let title = document.select(&title_selector).flat_map(|x| x.text()).collect::<String>();
    let content_selector = Selector::parse(".postMessage").unwrap();
    let content = document.select(&content_selector).map(|x| x.text().collect::<String>()).collect::<String>();
    let image_url = document.select(
      &Selector::parse(".fileThumb").unwrap()
    ).map(|x| x.attr("href").unwrap_or("No image posted")).collect::<String>();


    return ThreadReply{
      reply: content,
      subject: title,
      image_response: image_url,  // this image url will need link validation and url fixing  to be displayed to the user
      is_op: is_op.unwrap_or(false)
    }
  }

pub fn get_thread_responses(thread_content: &str) -> Vec<ThreadReply>{
  let threads_selector = Selector::parse(".postContainer").unwrap();
  let thread_page = Html::parse_document(&thread_content);
  return thread_page.select(&threads_selector)
  .map(|thread| {
    let is_op = thread.value().classes().collect::<String>().contains("opContainer");
    parse_html_reply(&thread.inner_html(), Some(is_op))
  })
  .collect();
}

  pub fn list_threads(content: &str) -> Vec<Thread>{
    let document = Html::parse_document(content);
    let threads_selector = Selector::parse(".thread").unwrap();
    let html_threads = document.select(&threads_selector).map(|x| x.inner_html());
    return html_threads.map(|thread| parse_html_thread(Html::parse_document(&thread))).collect();
  }
}