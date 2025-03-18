import { For } from 'solid-js';

import { SkinWithSpray } from './SkinSelect';

export interface SpriteSelectProps {
  skin: () => SkinWithSpray | undefined;
  value: () => string;
  onChange: (sprite: string) => void;
}

function asHumanReadable(name: string): string {
  switch (name) {
    case "STIN":
      return "Still";
    case "STIL":
      return "Still Left";
    case "STIR":
      return "Still Right";
    case "STGL":
      return "Still Left (glance back)";
    case "STGR":
      return "Still Right (glance back)";
    case "STLL":
      return "Still Left (look back)";
    case "STLR":
      return "Still Right (look back)";
    case "SLWN":
      return "Slow Driving";
    case "SLWL":
      return "Slow Driving Left";
    case "SLWR":
      return "Slow Driving Right";
    case "SLGL":
      return "Slow Driving Left (glance back)";
    case "SLGR":
      return "Slow Driving Right (glance back)";
    case "SLLL":
      return "Slow Driving Left (look back)";
    case "SLLR":
      return "Slow Driving Right (look back)";
    case "FSTN":
      return "Fast Driving";
    case "FSTL":
      return "Fast Driving Left";
    case "FSTR":
      return "Fast Driving Right";
    case "FSGL":
      return "Fast Driving Left (glance back)";
    case "FSGR":
      return "Fast Driving Right (glance back)";
    case "FSLL":
      return "Fast Driving Left (look back)";
    case "FSLR":
      return "Fast Driving Right (look back)";
    case "DRLN":
      return "Drifting Left, Steering Neutral";
    case "DRLO":
      return "Drifting Left, Steering Outwards";
    case "DRLI":
      return "Drifting Left, Steering Inwards";
    case "DRRN":
      return "Drifting Right, Steering Neutral";
    case "DRRO":
      return "Drifting Right, Steering Outwards";
    case "DRRI":
      return "Drifting Right, Steering Inwards";
    case "SPIN":
      return "Spinout";
    case "DEAD":
      return "Dead";
    case "SIGN":
      return "Finish Signpost";
    case "SIGL":
      return "Finish Signpost, Ironman Perfect";
    case "SSIG":
      return "\"working designs\" Signpost";
    case "XTRA":
      return "Wanted";
    case "TALK":
      return "Dialogue Icon";
    default:
      return name;
  }
}

const SpriteSelect = (props: SpriteSelectProps) => {
  return (
    <select
      value={props.value()}
      on:change={(ev) => {
        props.onChange(ev.target.value);
      }}
    >
      <For each={Array.from(props.skin()?.sprites.keys() ?? [])}>
        {(item, _index) => {
          return (
            <option value={item}>{asHumanReadable(item)}</option>
          );
        }}
      </For>
    </select>
  );
};

export default SpriteSelect;
