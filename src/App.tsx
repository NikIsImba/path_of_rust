import { Stage, Container, Sprite, Text } from '@pixi/react';

function App() {
  return (
    <div className="bg-yellow-800">
      <div className="p-10">
        <MyComponent />
      </div>
    </div>
  );
}

export default App;

export const MyComponent = () => {
  return (
    <Stage width={600} height={1222} options={{ background: 0xffffff }}>
      <Sprite
        image="https://pixijs.io/pixi-react/img/bunny.png"
        x={400}
        y={200}
        anchor={{ x: 0.5, y: 0.5 }}
      />

      <Container x={400} y={330}>
        <Text text="Hello World" anchor={{ x: 0.5, y: 0.5 }} />
      </Container>
    </Stage>
  );
};
