#![allow(non_snake_case)]

mod variants {
    pub mod cyoa;
    pub mod quiz;
}
pub mod toolbox;
pub mod types;


use crate::toolbox::readline;
use crate::variants::cyoa::cyoa;
use crate::variants::quiz::quiz;

fn main() {
    println!("Hello, welcome to this text-based adventure. This game will give you a clearer answer to the question: \"How has Canada's relationship with Britain evolved since 1914?\". In this game, you will be asked a series of questions, which you will answer by entering text into the console. You have two options: Play this game as a quiz, or as a choose your own adventure. For further intructions, please read the document at https://docs.google.com/document/d/1qqTJPUFDhEs_ZjiJtj5Lmf5Ay7bchljRF-a4GYj7Ki0/edit.");
    println!("Enter 1 to play the quiz, or 2 to play the choose your own adventure:");
    loop {
        let option = readline();
        if option == "1" {
            quiz(); // This is running the file in quiz.rs, so that the code is not all cluttered in this file.
        } else if option == "2" {
            cyoa(); // Same here, this area is generally just for directing the user to the gamemode they want to play.
        } else {
            println!("Please enter a valid option(1 or 2).");
            continue;
        }
        break;
    }
    println!("Sources:(Enter any key to continue)");
    let _ = readline();
    println!("Works Cited
    Arroyo, Martin. “,.” , - YouTube, 22 September 2021, https://ca.sports.yahoo.com/news/prime-minister-justin-trudeau-responds-142600253.html?guccounter=1&guce_referrer=aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbS8&guce_referrer_sig=AQAAAEIkfWGSbeyOxwuKzR_rMARFHSepQcSDz4HJFdS-bd-3ftInkU4eHKNcTw0UyoMEM8hkUZBmaoM67KM77Pova_fx. Accessed 8 January 2024.
    “Canada.” The Royal Family, https://www.royal.uk/canada. Accessed 21 December 2023.
    “Canadians Conflicted on Future Role of Monarchy as Half (54%) Say Canada Should End Ties to Monarchy.” Ipsos, 16 September 2022, https://www.ipsos.com/en-ca/news-polls/canadians-conflicted-on-future-role-of-monarchy. Accessed 21 December 2023.
    “Canadians Conflicted on Future Role of Monarchy as Half (54%) Say Canada Should End Ties to Monarchy.” Ipsos, 16 September 2022, https://www.ipsos.com/en-ca/news-polls/canadians-conflicted-on-future-role-of-monarchy. Accessed 22 December 2023.
    Corden, James. “Justin Trudeau tears up as he announces Queen Elizabeth II's death - 'one of my favourite people.'” YouTube, 9 September 2022, https://www.youtube.com/watch?v=ZnSrVIWxHAk. Accessed 8 January 2024.
    de Visser, John, and Jim Merrithew. “Québec Referendum (1980).” The Canadian Encyclopedia, 27 August 2013, https://www.thecanadianencyclopedia.ca/en/article/quebec-referendum-1980. Accessed 21 December 2023.
    “The First World War.” First World War (1914 – 1918) - Veterans Affairs Canada, 18 May 2022, https://www.veterans.gc.ca/eng/remembrance/wars-and-conflicts/first-world-war/. Accessed 21 December 2023.
    Gruending, Dennis. “Jean Chretien, Quebec referendum, 1995 – Great Canadian Speeches.” Great Canadian Speeches, 24 October 2020, https://greatcanadianspeeches.ca/2020/10/24/jean-chretien-quebec-referendum-1995/. Accessed 8 January 2024.
    Gruending, Dennis. “Pierre Trudeau, no to Quebec sovereignty, 1980 – Great Canadian Speeches.” Great Canadian Speeches, 5 May 2021, https://greatcanadianspeeches.ca/2021/05/05/pierre-trudeau-no-to-quebec-sovereignty-1980/. Accessed 8 January 2024.
    Mackenzie, Hector M. “Billion Dollar Gift.” The Canadian Encyclopedia, 6 February 2006, https://www.thecanadianencyclopedia.ca/en/article/billion-dollar-gift. Accessed 8 January 2024.
    McIntosh, Andrew. “The Great Flag Debate.” The Canadian Encyclopedia, 11 December 2019, https://www.thecanadianencyclopedia.ca/en/article/flag-debate. Accessed 8 January 2024.
    “Milestones: 1953–1960 - Office of the Historian.” History State Gov, https://history.state.gov/milestones/1953-1960/suez. Accessed 10 January 2024.
    “Military Service Act.” The Canadian Encyclopedia, https://www.thecanadianencyclopedia.ca/en/article/military-service-act. Accessed 27 December 2023.
    “Personnel Records of the First World War - Library and Archives Canada.” Personnel Records of the First World War - Library and Archives Canada, https://www.bac-lac.gc.ca/eng/discover/military-heritage/first-world-war/personnel-records/Pages/personnel-records.aspx#. Accessed 27 December 2023.
    Readers Digest. The Canadians at war, 1939/45. Readers Digest, 1986. Amazon, https://www.amazon.ca/Canadians-war-1939-45/dp/0888501455. Accessed 8 January 2024.
    “War Measures Act | Teachers' Zone.” Canadian Museum of History, 22 August 1914, https://www.historymuseum.ca/teachers-zone/lost-liberties/first-world-war/war-measures-act/#. Accessed 8 January 2024.
    ")
}
