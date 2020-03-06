import requests


if __name__ == "__main__":

    url = "http://localhost:8000/movies/"
    response = requests.get(url)
    data = response.json()

    print(response)
    print(data)
