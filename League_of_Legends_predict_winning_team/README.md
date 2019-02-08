# 리그 오브 레전드 승패 예측

본 프로젝트는 유명 AOS게임 리그 오브 레전드의 승패를 예측하는 프로젝트입니다.

---

### Requirements
  - requests
  - json

---
### Data Build
`python data_build/get_user_id.py`
`python data_build/get_game_id.py`
`python data_build/get_match_info.py`

> 분석에 필요한 데이터를 API로 요청하는 파일들입니다.
> 마스터 티어(Master tier)의 유저들의 게임 정보만을 가져옵니다.

> API 요청을 위해 [League of Legends 공식 홈페이지](https://developer.riotgames.com/api-keys.html)에서 API KEY를 받아서 상위 폴더에 **API_KEY.txt** 파일로 저장해주시기 바랍니다.

> 공식 API로 2분에 100건의 요청만 가능하기 때문에, 해당 과정은 상당한 시간이 소요됩니다.
> 따라서, nohup으로 백그라운드에서 실행하시면 좋습니다.

`data/match_info.json`
> 게임에 대한 정보들이 저장됩니다. [자세한 내용](https://developer.riotgames.com/api-methods/#match-v4/GET_getMatch)은 링크를 참조해주시기 바랍니다.
