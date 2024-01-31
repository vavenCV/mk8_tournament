import { AppBar, Box, Button, Container } from "@mui/material";
import "./App.css";

function AddTeam() {
    return (
        <Container sx={{ border: 1, borderRadius: 2, boxShadow: 5 }}>
            <h3>Add a team</h3>
            <Box>
                <Button>qweqwe</Button>
                <Button type="submit">submit</Button>
            </Box>
        </Container>
    );
}

function App() {
    return (
        <div>
            <AppBar position="absolute" sx={{ textShadow: 1 }}>
                <h2 style={{ textShadow: "1px 1px 1px #444",  fontFamily: "mariokart" }}>ADVANS MARIO KART TOURNAMENT</h2>
            </AppBar>
            <Container>
                <Box border={"black"}>
                    <AddTeam />
                </Box>
            </Container>
        </div>
    );
}

export default App;
