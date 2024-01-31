import {
  AppBar,
  Avatar,
  Box,
  Button,
  Container,
  Icon,
  List,
  ListItem,
  Paper,
  Tab,
  Tabs,
  TextField,
} from "@mui/material";
import "./App.css";
import mk8LogoUrl from "./assets/mk8-logo.png";
import React, { useEffect, useState } from "react";
import axios, { AxiosResponse } from "axios";

function AddTeam() {
  return (
    <Paper style={{ padding: "1px" }}>
      <Box color="secondary">
        <h4>Add team</h4>
      </Box>
      <TextField id="outlined-basic" label="Team ID" variant="outlined" />
      <Box m={1}>
        <Button color="primary">Submit</Button>
      </Box>
      {/* </Box> */}
    </Paper>
  );
}

function AppBarImage(props: { src: string }) {
  return (
    <Box
      component={"img"}
      src={props.src}
      width={"40px"}
      height={"40px"}
      m={2}
      marginY="auto"
    ></Box>
  );
}

function MKTourAppBar() {
  return (
    <AppBar position="absolute" sx={{ textShadow: 1 }}>
      <div style={{ display: "flex", margin: "auto", gap: 20 }}>
        <AppBarImage src={mk8LogoUrl} />
        <Box m={0}>
          <h2
            style={{
              textShadow: "1px 1px 0px #444",
              fontFamily: "mariokart",
              fontSize: "18pt",
              outlineWidth: "4px",
              outlineColor: "#000",
            }}
          >
            MARIO KART TOURNAMENT
          </h2>
        </Box>
      </div>
    </AppBar>
  );
}

function TeamList() {
  const [teams, setTeams] = useState([]);

  useEffect(() => {
    axios.get("127.0.0.1:5000/players").then((res: AxiosResponse) => {
        setTeams(res.data);
    }).catch((err) => {
        console.log(err);
    })
  })

  return (
    <Paper>
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
    <Paper>
      <List>
        <ListItem>qwe</ListItem>
      </List>
    </Paper>
  );
}

function MKTourTabs() {
  const [current, setCurrent] = useState(0);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setCurrent(newValue);
  };

  return (
    <Container>
      <Tabs value={current} onChange={handleChange} centered>
        <Tab label="Teams" />
        <Tab label="Players" />
        <Tab label="Leaderboard" />
      </Tabs>
      {current == 0 && <TeamList />}
      {current == 1 && <PlayerList />}
      {current == 2 && <Paper>TODO</Paper>}
    </Container>
  );
}

function App() {
  return (
    <div>
      <MKTourAppBar />
      <MKTourTabs />
      <AddTeam />
    </div>
  );
}

export default App;
