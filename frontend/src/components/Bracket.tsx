import { Paper } from "@mui/material";

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
        // fill="#5FAD41"
        // 2nd place orange
        // fill="#F5853F"
        // loser red
        // fill="#D0012E"
        // fillOpacity={1.0}

        // Undecided
        fillOpacity={0.0}
        //
        strokeWidth={1}
        stroke="#000"
        rx={1}
      ></rect>
      <text
        x={x + 9}
        y={y + BOX_HEIGHT / 2 + 1}
        // Decided
        // fill="#ffffffdd"
        // Undecided
        fill="black"
        fontSize={12}
        fontFamily="Arial"
        dominantBaseline={"middle"}
      >
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

function FaceOff({
  x,
  y,
  id,
  teams,
}: {
  x: number;
  y: number;
  id: string;
  teams: FaceOffTeams;
}) {
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
      <text
        x={x + BOX_WIDTH / 2}
        y={y}
        fontSize={14}
        fontFamily="Playfair Display"
        textAnchor="middle"
      >
        {id}
      </text>
      <LabelInABox x={x} y={yA} label={teams.teamA}></LabelInABox>
      <LabelInABox x={x} y={yB} label={teams.teamB}></LabelInABox>
      <LabelInABox x={x} y={yC} label={teams.teamC}></LabelInABox>
    </a>
  );
}

function Phase({
  x,
  phase_index,
  faceoffs,
}: {
  x: number;
  phase_index: string;
  faceoffs: FaceOffTeams[];
}) {
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
        <FaceOff
          x={x}
          y={75 + yOffset * i}
          id={`${phase_index}-${String.fromCharCode(charCodeA + i)}`}
          teams={faceoff}
        ></FaceOff>
      ))}
    </>
  );
}

function Bracket() {
  const PHASE_SPACING = BOX_WIDTH * 2;
  const CANVAS_WIDTH = BOX_WIDTH * 4 + (PHASE_SPACING / 2) * 3 + 100; // 100 = padding by 50 each side just in case
  const CANVAS_HEIGHT = 700;

  // TODO:
  // + add TBD logic for matches where opponents are still unknown
  // + update bracket when a match has occurred
  // + bonus: when hovering over match, show lines to the previous matches and to the next for contestants

  return (
    <Paper
      id="BracketPaper"
      elevation={4}
      style={{ overflow: "scroll", scrollbarWidth: "thin", margin: "1em" }}
    >
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
          faceoffs={[{ teamA: "GRE", teamB: "LYS", teamC: "CAC" }]}
        ></Phase>
      </svg>
    </Paper>
  );
}

export default Bracket;