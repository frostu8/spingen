import { createMemo, For } from 'solid-js';

import { SkinWithSpray } from './SkinSelect';

export interface FrameSelectProps {
  skin: () => SkinWithSpray | undefined;
  sprite: () => string;
  value: () => string;
  onChange: (sprite: string) => void;
}

const FrameSelect = (props: FrameSelectProps) => {
  const frames = createMemo<string[] | undefined>((_old) => {
    const skin = props.skin();
    if (!skin) {
      return;
    }

    const sprite = props.sprite();
    return skin.sprites.get(sprite)?.frames;
  });

  return (
    <select
      value={props.value()}
      on:change={(ev) => {
        props.onChange(ev.target.value);
      }}
    >
      <For each={frames()}>
        {(item, _index) => {
          return (
            <option value={item}>{item}</option>
          );
        }}
      </For>
    </select>
  );
};

export default FrameSelect;
