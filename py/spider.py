import requests
from bs4 import BeautifulSoup

eshop_hk = "https://store.nintendo.com.hk/"
api_root = ""

response = requests.get(eshop_hk)

soup = BeautifulSoup(response.text,"html5lib")

for item in soup.find_all(name='div',attrs={"class":"category-product-item"}):
    image = item.find_all("img")[0]
    image_src = image['src']
    title = image['alt']
    price = item.find_all(name= "span", attrs={"class": "price-wrapper"})[0]['data-price-amount']
    price = int(price)
    detail_link = item.find_all(name= "a", attrs={"class" : "category-product-item-title-link"})[0]['href']
    detail_page = requests.get(detail_link)

    desc_soup = BeautifulSoup(detail_page.text,"html5lib")
    desc = desc_soup.find_all(name="div", attrs={"class": "product attribute description"})[0]
    desc_paragraph_list = desc.find_all('p')
    desc = ""
    for desc_iter in desc_paragraph_list:
        desc += desc_iter.getText()
    data = {
        "gamename": title,
        "price": price,
        "link": detail_link,
        "image": image_src,
        "desc": desc,
    }
    requests.post(url = api_root + "/api/spider/add_game",json=data)


