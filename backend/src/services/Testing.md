# Faceoff creation

## Teams

First create 3 teams
```
curl -i -X POST -H "Content-Type: application/json" -d '{"player_names":["[GRE] p1", "[GRE] p2", "[GRE] p3", "[GRE] p4"]}' http://localhost:5000/teams
curl -i -X POST -H "Content-Type: application/json" -d '{"player_names":["[LYS] p1", "[LYS] p2", "[LYS] p3", "[LYS] p4"]}' http://localhost:5000/teams
curl -i -X POST -H "Content-Type: application/json" -d '{"player_names":["[SOF] p1", "[SOF] p2", "[SOF] p3", "[SOF] p4"]}' http://localhost:5000/teams
```
Then get the ids

```
curl -s -X GET -H "Content-Type: application/json" http://localhost:5000/teams | jq '.[] | .id'
```

## Faceoff

```
curl -i -X POST -H "Content-Type: application/json" -d '{"race_number":6, "team_ids": [-259569, 580317584, 2117927201]}' http://localhost:5000/faceoffs
```

This will create all races with empty points, to be created later

## Races

```
curl -i -X PUT -H "Content-Type: application/json" -d '{"race_points": [{"player_id": 1555480604, "points": 15}, {"player_id": 1431639362, "points": 12}, {"player_id": 1237765841, "points": 10}, {"player_id": -112902154, "points": 8}, {"player_id": -201583806, "points": 7}, {"player_id": -225324610, "points": 5}, {"player_id": -326015235, "points": 5}, {"player_id": -364038806, "points": 4}, {"player_id": -469872453, "points": 3}, {"player_id": -880137878, "points": 2}, {"player_id": -959482049, "points": 1}, {"player_id": -1026797181, "points": 0}]}' http://localhost:5000/races/1625031395 
```


```
curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/races/-1338956250
```
