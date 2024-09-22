mod requester;
mod seeker;
mod data_structures;
mod config;


pub fn fix_thread_url(thread_url: String) -> String{
  if thread_url.contains("//") && !thread_url.contains("https://"){
    return thread_url.replace("//", "https://");
  }
  else{ return thread_url;}
}

fn thread_test(){
  let homebod = requester::requester::get_url(String::from("https://boards.4chan.org/g/catalog")).unwrap();
  let threads = seeker::list_threads(&homebod);
  // let first_url = threads[0].url.clone();
  // seeker::seeker::parse_html_reply(&firstthread, Some(false));
  for mut thread in threads{
    let thread_content = requester::requester::get_url(fix_thread_url(thread.url.clone())).unwrap();
    println!("got the thread content to: {}", &thread.title);
    thread.reply = seeker::get_thread_responses(&thread_content);
    println!("Thread image at catalog: {}", thread.image_href);
  }
}

fn config_test(){
  config::create_config_ifn_exists();
  let mut cn = config::get_config();
  println!("got config with {} host", cn.host);
  cn.user = "test".to_string();
  config::save_config(cn);
  println!("Saved new config settings");
}


fn main() {
  config_test();
}
