import sys
sys.path.append('../')

from utility import *

# 유저의 accountId로 게임 정보를 요청하는 함수
def get_game_id(account_id, API_KEY):
    api_url = 'https://kr.api.riotgames.com/lol/match/v4/matchlists/by-account/%s?api_key=%s'%(account_id, API_KEY)
    match_info_dic = requests.get(api_url).json()
    game_ls = match_info_dic['matches']

    game_id_ls = []

    for game in game_ls:
        queue = game['queue'] # 해당 게임의 종류 : 소환사의 협곡 5:5는 420(랭크), 430(일반)
        game_id = game['gameId']

        if queue in [420, 430]:
            game_id_ls.append(game_id)

    time.sleep(1.5)
    return match_info_dic, game_id_ls


def main():
    print('게임 정보를 수집합니다')
    API_KEY = read_api_key('../API_KEY.txt')

    user_info_dic = read_json('../data/summoner_info.json')

    try:
        game_id_dic = read_json('../data/game_id.json')
        print('사전에 생성한 게임 정보를 불러왔습니다.')
        print('추가적으로 업데이트를 진행합니다.')
    except:
        game_id_dic = {
            'user' : {},
            'game_id' : [],
        }
        print('저장된 유저 정보가 없습니다.')
        print('새롭게 유저 정보를 기록합니다.')

    # 유저의 게임 정보를 API로 요청, gameId는 추후에 게임(match)의 세부적인 정보를 요청하기 위해 사용된다.
    print('유저가 치른 게임(match)의 id를 요청합니다.')
    print('API 요청은 2분 당 100회로 제한되어 있습니다.')
    for user_name in user_info_dic.keys():
        try:
            account_id = user_info_dic[user_name]['accountId']

            # 저장된 게임 정보가 없는 유저만 API로 요청한다
            if not user_name in game_id_dic['user']:
                temp_user_dic, temp_game_id_ls = get_game_id(account_id, API_KEY)

                # 저장
                game_id_dic['user'][user_name] = temp_user_dic
                game_id_dic['game_id'] += temp_game_id_ls

                # 중복 제거
                game_id_dic['game_id'] = list(set(game_id_dic['game_id']))
        except: pass


        # 현재 진행상황 보고
        n_user = len(game_id_dic['user'])    # 저장된 유저의 id 수
        if n_user%100 == 0:
            # 게임 id 사전을 업데이트,
            save_json(game_id_dic, '../data/game_id.json')
            print('%s 번째 소환사의 정보를 저장하였습니다.'%n_user)
    return

if __name__ == '__main__':
    main()
