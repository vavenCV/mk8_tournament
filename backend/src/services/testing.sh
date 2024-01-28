#!/bin/bash

# Given input JSON array of player information
input_json='[{"id":-1026797181,"team_id":790284319,"name":"[SOF] p3"},{"id":-959482049,"team_id":790284319,"name":"[SOF] p1"},{"id":-880137878,"team_id":1650047014,"name":"[GRE] p4"},{"id":-469872453,"team_id":1439906126,"name":"[LYS] p1"},{"id":-364038806,"team_id":1650047014,"name":"[GRE] p2"},{"id":-326015235,"team_id":1439906126,"name":"[LYS] p2"},{"id":-225324610,"team_id":1650047014,"name":"[GRE] p1"},{"id":-201583806,"team_id":1650047014,"name":"[GRE] p3"},{"id":-112902154,"team_id":790284319,"name":"[SOF] p2"},{"id":1237765841,"team_id":790284319,"name":"[SOF] p4"},{"id":1431639362,"team_id":1439906126,"name":"[LYS] p3"},{"id":1555480604,"team_id":1439906126,"name":"[LYS] p4"}]'

# Convert the input JSON array to the desired format
output_json=$(echo "$input_json" | jq -c '[.[] | {player_id, points: 0, name}]')

# Create the final JSON object with the "race_points" key
final_json="{\"race_points\": $output_json}"

# Print the resulting JSON
echo "$final_json"
