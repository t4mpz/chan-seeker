mod requester;
mod seeker;


fn main() {
    let homebod = requester::requester::get_url(String::from("https://boards.4chan.org/g/catalog")).unwrap();
    let threads = seeker::seeker::list_threads(&homebod);
    // let first_url = threads[0].url.clone();
    // seeker::seeker::parse_html_reply(&firstthread, Some(false));
    for mut thread in threads{
        let thread_content = requester::requester::get_url(requester::requester::fix_thread_url(thread.url.clone())).unwrap();
        println!("got the thread content to: {}", &thread.title);
        thread.reply = seeker::seeker::get_thread_responses(&thread_content);
        println!("Thread image at catalog: {}", thread.image_href);
    }


}
