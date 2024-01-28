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
curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/races/-1338956250
```
