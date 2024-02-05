import {
  Box,
  Container,
  List,
  ListItem,
  Paper,
  Tab,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Tabs,
} from "@mui/material";
import MKTourAppBar from "./components/AppBar";
import React, { useEffect, useState } from "react";
import { AxiosResponse } from "axios";
import api from "./api";
import theme from "./theme.ts";
import Bracket from "./components/Bracket.tsx";

interface Team {
  id: number;
  player_ids: number[];
}

function TeamList() {
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    api
      .get("/teams")
      .then((res: AxiosResponse) => {
        // console.debug(res);
        setTeams(res.data);
      })
      .catch((err) => {
        // console.debug(err);
      });
  }, []);

  return (
    <Paper>
      <List>
        {teams.map((team) => {
          return <ListItem key={team.id}>{team.id}</ListItem>;
        })}
      </List>
    </Paper>
  );
}

interface Player {
  team_id: number;
  name: string;
  id: number;
}

function PlayerList({ players, teams }: { players: Player[]; teams: Team[] }) {
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow sx={{ bgcolor: theme.palette.primary.main }}>
            <TableCell sx={{ color: theme.palette.common.white }}>
              Name
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Team
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Total score
            </TableCell>
            <TableCell align="right" sx={{ color: theme.palette.common.white }}>
              Ranking
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {players.map((player) => (
            <TableRow
              key={player.id}
              sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
            >
              <TableCell>{player.name}</TableCell>
              {/* TODO replace team id by team name */}
              <TableCell align="right">
                {teams.filter((t) => t.id == player.team_id)[0].id}
              </TableCell>
              <TableCell align="right">
                {/* TODO replace id by score */}
                {/* {player.score} */} {player.id}
              </TableCell>
              <TableCell align="right">
                {/* TODO replace id by score */}
                {players.sort((a, b) => a.id - b.id).indexOf(player) + 1}
              </TableCell>
            </TableRow>
          ))}
          <TableRow></TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  );
}

function MKTourTabs() {
  // Current tab index
  const [current, setCurrent] = useState(0);
  // List of players fetched from DB
  const [players, setPlayers] = useState<Player[]>([]);
  // List of teams fetched from DB
  const [teams, setTeams] = useState<Team[]>([]);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
      setCurrent(newValue);
  };

  useEffect(() => {
    api
      .get("/players")
      .then((res: AxiosResponse) => {
        // console.log(res);
        setPlayers(res.data);
      })
      .catch((err) => {
        // console.log(err);
      });
  }, []);

  useEffect(() => {
    api
      .get("/teams")
      .then((res: AxiosResponse) => {
        // console.log(res);
        setTeams(res.data);
      })
      .catch((err) => {
        // console.log(err);
      });
  }, []);

  return (
    <Container sx={{ marginTop: "1em" }}>
      <Box width="90%" style={{ margin: "auto" }}>
        <Tabs
          value={current}
          onChange={handleChange}
          centered
          sx={{ marginBottom: "1em" }}
          variant="fullWidth"
        >
          <Tab label="Bracket" />
          <Tab label="Teams" />
          <Tab label="Players" />
        </Tabs>
        {current == 0 && <Bracket />}
        {current == 1 && <TeamList />}
        {current == 2 && <PlayerList players={players} teams={teams} />}
      </Box>
    </Container>
  );
}

function App() {
    return (
        <div id="AppDiv" style={{ width: "100%", height: "100%", display: "flex", flexDirection: "column" }}>
            <MKTourAppBar />
            <MKTourTabs />
        </div>
    );
}

export default App;
