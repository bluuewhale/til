import sys
sys.path.append('../')

from utility import *

# game_id(게임 번호)를 입력하면, 게임에 대한 정보를 요청하는 함수
def get_match_info(game_id, API_KEY):
    api_url = 'https://kr.api.riotgames.com/lol/match/v4/matches/%s?api_key=%s'%(game_id, API_KEY)
    match_dic = requests.get(api_url).json()
    time.sleep(1.3)
    return match_dic

def main():
    print('게임 정보를 수집합니다')
    API_KEY = read_api_key('../API_KEY.txt')

    # game_id(게임 번호) 불러오기
    game_id_ls = read_json('../data/game_id.json')['game_id']

    try:
        match_info_dic = read_json('../data/match_info.json')
        print('사전에 생성한 게임 정보를 불러왔습니다.')
        print('추가적으로 업데이트를 진행합니다.')
    except:
        match_info_dic = {
            'game_id' : [],
            'matches' : [],
        }
        print('저장된 유저 정보가 없습니다.')
        print('새롭게 유저 정보를 기록합니다.')


    for game_id in game_id_ls:
        # 중복되지 않은 게임만 정보를 요청
        if not game_id in match_info_dic['game_id']:
            match_info_dic['game_id'].append(game_id)
            match_info_dic['matches'].append(get_match_info(game_id, API_KEY))

        # 현재 진행상황 보고
        n_game = len(match_info_dic['game_id'])    # 저장된 게임의 수
        if n_game%100 == 0:
            # 게임 id 사전을 업데이트,
            save_json(match_info_dic, '../data/match_info.json')
            print('%s 번째 소환사의 정보를 저장하였습니다.'%n_game)
    return

if __name__ == '__main__':
    main()
