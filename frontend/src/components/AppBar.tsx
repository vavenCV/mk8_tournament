import { AppBar, Box } from "@mui/material";
import mk8LogoUrl from "../assets/mk8-logo.png";

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
    <AppBar position="sticky" sx={{ textShadow: 1, m: 0 }}>
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

export default MKTourAppBar;
