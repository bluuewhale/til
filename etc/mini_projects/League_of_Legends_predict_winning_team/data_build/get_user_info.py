import sys
sys.path.append('../')

import requests
import os
from utility import *
from collections import defaultdict

# 마스터 리그에 소속된 유저들의 이름(summonerName)을 수집하는 함수
def get_master_league_user_names(API_KEY):
    api_url = 'https://kr.api.riotgames.com/lol/league/v4/masterleagues/by-queue/RANKED_SOLO_5x5?api_key=%s'%API_KEY
    json_dic = requests.get(api_url).json()['entries']

    summoner_name_ls = []
    for user_dic in json_dic:
        summoner_name_ls.append(user_dic['summonerName'])

    return summoner_name_ls


# 소환사 이름으로 유저에 대한 정보들을 반환하는 함수
# 게임 정보를 요청하기 위해 사용된다.
def get_user_info(summoner_name, API_KEY):
    api_url = 'https://kr.api.riotgames.com/lol/summoner/v4/summoners/by-name/%s?api_key=%s'%(summoner_name, API_KEY)
    json_dic = requests.get(api_url).json()
    time.sleep(1.5)
    return json_dic


def main():
    # get API_KEY
    print('유저 정보를 수집합니다')
    API_KEY = read_api_key('../API_KEY.txt')

    # working directory에 data 폴더가 없으면 생성해준다.
    if not os.path.exists('../data'):
        os.mkdir('../data')

    # 마스터 리그의 소환사 이름 정보를 가져옴
    # 해당 정보가 있으면 과정 생략
    summoner_name_ls = get_master_league_user_names(API_KEY)

    # 유저 정보를 기록할 사전 생성, 혹은 기존에 사전을 추가적으로 업데이트
    try:
        summoner_info_dic = read_json('../data/summoner_info.json')
        print('사전에 생성한 유저 정보를 불러왔습니다.')
        print('추가적으로 업데이트를 진행합니다.')
    except:
        summoner_info_dic = {}
        print('저장된 유저 정보가 없습니다.')
        print('새롭게 유저 정보를 기록합니다.')

    # 유저 정보를 API로 요청, accountId는 추후에 게임(match)정보를 요청하기 위해 사용된다.
    print('유저 정보를 API로 요청합니다.')
    print('API 요청은 2분 당 100회로 제한되어 있습니다.')
    while summoner_name_ls:
        name = summoner_name_ls.pop()

        # 해당 유저의 정보가 없는 경우, API로 데이터를 요청
        if not name in summoner_info_dic.keys():
            summoner_info_dic[name] = get_user_info(name, API_KEY)

        # 현재 진행상황 보고
        if len(summoner_info_dic)%100 == 0:
            # 유저 정보 사전을 업데이트,
            save_json(summoner_info_dic, '../data/summoner_info.json')
            print('%s 번째 소환사의 정보를 저장하였습니다.'%len(summoner_info_dic))

if __name__ == '__main__':
    main()
