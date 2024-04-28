import {
  Stage,
  Container,
  Sprite,
  Text,
  PixiComponent,
  applyDefaultProps,
  useApp,
} from '@pixi/react';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from './@/components/ui/resizable';
import { Viewport } from 'pixi-viewport';

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
                <Stage
                  width={40000}
                  height={1222}
                  options={{ background: 0xff0000 }}
                >
                  <MyComponent />
                </Stage>
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
  const app = useApp();
  return (
    <PixiViewport app={app} wor>
      <Sprite
        image='https://pixijs.io/pixi-react/img/bunny.png'
        x={400}
        y={200}
        anchor={{ x: 0.5, y: 0.5 }}
      />

      <Container x={400} y={330}>
        <Text text='Hello World' anchor={{ x: 0.5, y: 0.5 }} />
      </Container>
    </PixiViewport>
  );
};

const PixiViewport = PixiComponent('PixiViewport', {
  create: (props) => {
    return new Viewport({
      screenWidth: window.innerWidth,
      screenHeight: window.innerHeight,
      worldWidth: 40000,
      worldHeight: 1222,
      interaction: props.app.renderer.plugins.interaction,
    });
  },

  applyProps: (instance, oldProps, newProps) => {
    applyDefaultProps(instance, oldProps, newProps);
  },

  didMount: (instance, parent) => {
    instance.drag({ pressDrag: true }).pinch().wheel().decelerate();
    parent.addChild(instance);
  },

  willUnmount: (instance, parent) => {
    parent.removeChild(instance);
  },

  config: {
    destroy: true,
    destroyChildren: true,
  },
});
