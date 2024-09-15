import requests
from bs4 import BeautifulSoup
import lmdb 

def get_book_information(isbn: str):
    r = requests.get("https://www.librarything.com/work/" + isbn)

    soup = BeautifulSoup(r.text, "html.parser")

    baseTag = soup.body.find("div", class_="summary").table.find_all("td")
    title = baseTag[1].h1
    author = baseTag[1].h2.a
    summary = baseTag[1].find_all("div")[2].table.find_all("tr")[2].td.div
    cover = baseTag[0].div.find_all("img")[1]

    print(title.get_text())
    print(author.get_text())
    print(summary.get_text())
    print(cover.get("src"))
    
