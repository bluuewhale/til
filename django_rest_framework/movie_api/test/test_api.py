import requests
import unittest


class MovieAPITest(unittest.TestCase):
    """ Moive API Test """

    def test_index(self):
        url = "http://localhost:8000/movies/api"

        response = requests.get(url)
        self.assertEqual(response.status_code, 200)

        data = response.json()
        self.assertIsInstance(data, list)

    def test_detail(self):
        url = "http://localhost:8000/movies/api"

        response = requests.get(url)
        self.assertEqual(response.status_code, 200)

        data = response.json()
        self.assertIsInstance(data, dict)

    def test_update(self):
        raise NotImplementedError()

    def test_delete(self):
        raise NotImplementedError()

    def test_create(self):
        raise NotImplementedError()


if __name__ == "__main__":

    unittest.main()
