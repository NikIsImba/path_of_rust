import { Stage, Container, Sprite, Text } from '@pixi/react';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './@/components/ui/resizable';

function App() {
  return (
    <div className='h-screen w-screen'>
      <ResizablePanelGroup direction='horizontal'>
        <ResizablePanel defaultSize={85}>
          <ResizablePanelGroup direction='vertical'>
            <ResizablePanel defaultSize={5}>
              <div className='h-full w-full bg-yellow-200'>
                <div>Topbar</div>
              </div>
            </ResizablePanel>
            <ResizableHandle disabled={true} className='bg-black' />
            <ResizablePanel defaultSize={95}>
              <div className='h-full w-full'>
                <MyComponent />
              </div>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>
        <ResizableHandle disabled={true} className='bg-black' />
        <ResizablePanel defaultSize={15}>
          <div className=' h-full w-full bg-yellow-800'>
            <div>Sidebar</div>
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;

export const MyComponent = () => {
  return (
    <Stage width={3222} height={1222} options={{ background: 0xff0000 }}>
      <Sprite
        image='https://pixijs.io/pixi-react/img/bunny.png'
        x={400}
        y={200}
        anchor={{ x: 0.5, y: 0.5 }}
      />

      <Container x={400} y={330}>
        <Text text='Hello World' anchor={{ x: 0.5, y: 0.5 }} />
      </Container>
    </Stage>
  );
};
