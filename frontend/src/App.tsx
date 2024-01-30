import './App.css'


function AddTeam({arg}: {arg: string}) {
  return <p>{arg}</p>
}


function App() {
  return (
    <div>
      <p>Hello world!</p>
        <AddTeam arg="4" />
    </div>
  )
}

export default App
