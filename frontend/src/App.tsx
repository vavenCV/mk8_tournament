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
import "./App.css";
import MKTourAppBar from "./components/AppBar";
import React, { useEffect, useState } from "react";
import { AxiosResponse } from "axios";
import api from "./api";
import theme from "./theme.ts";


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

function AppBarImage(props: { src: string }) {
    return <Box component={"img"} src={props.src} width={"40px"} height={"40px"} m={2} marginY="auto"></Box>;
}

function MKTourAppBar() {
    return (
        <>
            <AppBar position="sticky" sx={{ textAlign: "center" }}>
                <div style={{ display: "flex", margin: "auto", gap: 20 }}>
                    <AppBarImage src={mk8LogoUrl} />
                    <Box m={0}>
                        <h2
                            style={{
                                // textShadow: "1px 1px 2px #888",
                                fontFamily: "mariokart",
                                fontSize: "18pt",
                            }}
                        >
                            MARIO KART TOURNAMENT
                        </h2>
                    </Box>
                </div>
            </AppBar>
        </>
    );
}

function TeamList() {
    const [teams, setTeams] = useState([]);

    useEffect(() => {
        axios
            .get("127.0.0.1:5000/players")
            .then((res: AxiosResponse) => {
                setTeams(res.data);
            })
            .catch((err) => {
                console.log(err);
            });
    });

    return (
        <Paper elevation={4} sx={{ margin: 2 }}>
            <List>
                <ListItem>GRE</ListItem>
                <ListItem>LYS</ListItem>
                <ListItem>CAC</ListItem>
            </List>
        </Paper>
    );
}

function PlayerList() {
    const [players, setPlayers] = useState([]);

    return (
        <Paper elevation={4} sx={{ margin: 2 }}>
            <List>
                <ListItem>qwe</ListItem>
            </List>
        </Paper>
    );
}

// Constants for entire bracket display
const BOX_WIDTH = 125;
const BOX_HEIGHT = 20;

function LabelInABox({ x, y, label }: { x: number; y: number; label: string }) {

    // TODO if results are known, bg color is red/orange/green and text is white
    // if match hasn't been played yet, bg color is white and fg color is black

    return (
        <>
            <rect
                x={x}
                y={y}
                width={BOX_WIDTH}
                height={BOX_HEIGHT}
                // winner green
                fill="#5FAD41"
                // 2nd place orange
                // fill="#F5853F"
                // loser red
                // fill="#D0012E"
                fillOpacity={1.0}
                strokeWidth={1}
                stroke="#000"
                rx={1}
            ></rect>
            <text x={x + 9} y={y + BOX_HEIGHT / 2 + 1} fill="#ffffffdd" fontSize={12} fontFamily="Arial" dominantBaseline={"middle"}>
                {label}
            </text>
        </>
    );
}

interface FaceOffTeams {
    teamA: string;
    teamB: string;
    teamC: string;
}

function FaceOff({ x, y, id, teams }: { x: number; y: number; id: string; teams: FaceOffTeams }) {
    const yA = y + 5;
    const yB = y + 5 + BOX_HEIGHT;
    const yC = y + 5 + BOX_HEIGHT * 2;
    return (
        <a
            href="#"
            onClick={() => {
                alert();
            }}
        >
            <text x={x + BOX_WIDTH / 2} y={y} fontSize={14} fontFamily="Playfair Display" textAnchor="middle">
                {id}
            </text>
            <LabelInABox x={x} y={yA} label={teams.teamA}></LabelInABox>
            <LabelInABox x={x} y={yB} label={teams.teamB}></LabelInABox>
            <LabelInABox x={x} y={yC} label={teams.teamC}></LabelInABox>
        </a>
    );
}

function Phase({ x, phase_index, faceoffs }: { x: number; phase_index: string; faceoffs: FaceOffTeams[] }) {
    const yOffset = 100;
    const charCodeA = "A".charCodeAt(0);
    console.log(phase_index);
    return (
        <>
            <text
                fontFamily="Roboto"
                fontSize={15}
                fontWeight={600}
                x={x + BOX_WIDTH / 2}
                y={50}
                textAnchor="middle"
                // dominantBaseline={"middle"}
            >
                Phase {phase_index}
            </text>
            {/* <FaceOff x={x} y={75 + yOffset * 0} id="I-A" teams={faceoffs[0]}></FaceOff>
            <FaceOff x={x} y={75 + yOffset * 1} id="I-B" teams={faceoffs[1]}></FaceOff>
            <FaceOff x={x} y={75 + yOffset * 2} id="I-C" teams={faceoffs[2]}></FaceOff>
            <FaceOff x={x} y={75 + yOffset * 3} id="I-D" teams={faceoffs[3]}></FaceOff>
            <FaceOff x={x} y={75 + yOffset * 4} id="I-E" teams={faceoffs[3]}></FaceOff> */}
            {faceoffs.map((faceoff, i) => (
                <FaceOff x={x} y={75 + yOffset * i} id={`${phase_index}-${String.fromCharCode(charCodeA + i)}`} teams={faceoff}></FaceOff>
            ))}
        </>
    );
}

function Bracket() {
    const PHASE_SPACING = BOX_WIDTH * 2;
    const CANVAS_WIDTH = BOX_WIDTH * 4 + PHASE_SPACING / 2 * 3 + 100; // 100 = padding by 50 each side just in case
    const CANVAS_HEIGHT = 700;

    // TODO:
    // + add TBD logic for matches where opponents are still unknown
    // + update bracket when a match has occurred
    // + bonus: when hovering over match, show lines to the previous matches and to the next for contestants

    return (
        <Paper id="BracketPaper" elevation={4} style={{ overflow: "scroll", scrollbarWidth: "thin", margin: "1em" }}>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                // Have to set the svg size manually so that the overflow scrolling works
                style={{ width: CANVAS_WIDTH, height: CANVAS_HEIGHT }}
                // So we can have clean 1-pixel strokes:
                transform="translate(-0.5 -0.5)"
            >
                {/* Phase I */}
                <Phase
                    x={50 + PHASE_SPACING * 0}
                    phase_index="I"
                    faceoffs={[
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                    ]}
                ></Phase>
                {/* Phase II */}
                <Phase
                    x={50 + PHASE_SPACING * 1}
                    phase_index="II"
                    faceoffs={[
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                    ]}
                ></Phase>
                {/* Phase III */}
                <Phase
                    x={50 + PHASE_SPACING * 2}
                    phase_index="III"
                    faceoffs={[
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                    ]}
                ></Phase>
                {/* Phase IV */}
                <Phase
                    x={50 + PHASE_SPACING * 3}
                    phase_index="IV"
                    faceoffs={[
                        { teamA: "GRE", teamB: "LYS", teamC: "CAC" },
                    ]}
                ></Phase>
            </svg>
        </Paper>
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
        {current == 0 && <Paper>TODO BRACKET</Paper>}
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
