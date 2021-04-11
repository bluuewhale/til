import json
import pickle
import time
import requests


# API키 불러오기
def read_api_key(path_to_key):
    with open(path_to_key, 'r') as f:
        API_KEY = f.readline().replace('\n', '')
    return API_KEY

# json 형식 데이터 저장
def save_json(objec, path_to_file):
    return json.dump(objec, open(path_to_file, 'w', encoding='utf-8'), ensure_ascii=False)

# json 형식 데이터 로딩
def read_json(path_to_file):
    return json.load(open(path_to_file, 'r'))

# pickle 형식으로 저장
def save_pickle(objec, path_to_pickle):
    pickle.dump(objec, open(path_to_pickle, 'wb'))
    return

# pickle 형식 파일 로딩
def read_pickle(path_to_pickle):
    return pickle.load(open(path_to_pickle, 'rb'))
