import { For, untrack, createEffect, createSignal, Show, createMemo } from 'solid-js';

import DisplaySkin from './DisplaySkin.tsx';
import { SkinWithSpray } from './SkinSelect.tsx';
import SpriteSelect from './SpriteSelect.tsx';
import FrameSelect from './FrameSelect.tsx';

export interface ViewSkinProps {
  skin: () => SkinWithSpray | undefined;
}

export enum SpriteScale {
  X1 = "1x",
  X2 = "2x",
  X3 = "3x",
  X4 = "4x",
  X6 = "6x",
  X8 = "8x",
}

function skinClass(skin: SkinWithSpray) {
  const CLASSES = [
    "Class A", "Class B", "Class C", "Class D", "Class E", "Class F", "Class G", "Class H",
    "Class I",
  ];

  let x = Math.min((Math.max(skin.kartspeed, 1) - 1) / 3, 2);
  let y = Math.min((Math.max(skin.kartweight, 1) - 1) / 3, 2);

  return CLASSES[y * 3 + x];
}

function scaleToNumber(scale: SpriteScale) {
  switch (scale) {
    case SpriteScale.X1:
      return 1.0;
    case SpriteScale.X2:
      return 2.0;
    case SpriteScale.X3:
      return 3.0;
    case SpriteScale.X4:
      return 4.0;
    case SpriteScale.X6:
      return 6.0;
    case SpriteScale.X8:
      return 8.0;
    default:
      throw new Error("invalid scale: " + scale);
  }
}

const ALLOWED_SCALES: SpriteScale[] = [
  SpriteScale.X1,
  SpriteScale.X2,
  SpriteScale.X3,
  SpriteScale.X4,
  SpriteScale.X6,
  SpriteScale.X8,
];

const ViewSkin = (props: ViewSkinProps) => {
  const [sprite, setSprite] = createSignal<string>("STIN");
  const [frame, setFrame] = createSignal<string>("A");
  const [scale, setScale] = createSignal<SpriteScale>(SpriteScale.X1);

  const memoizedSprite = createMemo(() => sprite());

  const options = () => {
    return {
      sprite: sprite(),
      frame: frame(),
      scale: scaleToNumber(scale()),
    };
  };

  const onResetSpray = () => {
    const skin = props.skin();
    if (skin) {
      skin.setSpray(null);
    }
  };

  createEffect(() => {
    // on sprite change, reset frame to a
    const newSprite = memoizedSprite();
    const currentFrame = untrack(() => frame());
    const skin = props.skin();

    if (skin) {
      const sprite = skin.sprites.get(newSprite);

      if (sprite) {
        if (sprite.frames.findIndex((frame) => frame === currentFrame) < 0) {
          // sprite does not exist, reset
          setFrame(sprite.frames[0]);
        }
      } else {
        // reset sprite and frame to defaults
        setSprite("STIN");
        setFrame("A");
      }
    }
  })

  return (
    <section class="skin-show">
      <Show when={props.skin()}>
        <DisplaySkin skin={props.skin} options={options}/>
        <div class="skin-show-controls">
          <SpriteSelect
            skin={props.skin}
            value={sprite}
            onChange={setSprite}
          />
          <FrameSelect
            skin={props.skin}
            sprite={sprite}
            value={frame}
            onChange={setFrame}
          />
          <select
            on:change={(ev) => {
              setScale(ev.target.value);
            }}
            value={scale()}
          >
            <For each={ALLOWED_SCALES}>
              {(item, _index) => {
                return (
                  <option value={item}>{item}</option>
                );
              }}
            </For>
          </select>
          <button on:click={onResetSpray}>
              { "Use Preferred Spray" }
          </button>
        </div>
        <p>
          { "To save this: Right-click → Save Image As" }
          <br/>
          { "Showing " }
          <strong>
              {props.skin()!.realname.replace(/_/g, ' ')}
          </strong>
          { ", a "}
          <strong>
              {skinClass(props.skin()!)}
          </strong>
          { " (s" }
          {props.skin()!.kartspeed}
          { ", w" }
          {props.skin()!.kartweight}
          { ") driver." }
        </p>
      </Show>
    </section>
  );
};

export default ViewSkin;
