export interface HeaderProps { onFile: (file: File) => void; };

const Header = (props: HeaderProps) => {
  let fileDialog!: HTMLInputElement;

  let onInputFile = (ev: Event) => {
    let target = ev.target as HTMLInputElement;

    if (target.files) {
      for (const file of target.files) {
        props.onFile(file);
      }
    }
  };

  return (
    <header>
      <a
        href="https://github.com/frostu8/spingen"
        rel="noopener"
        target="_blank"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
      </a>
      <a
        href="https://github.com/frostu8/spingen/blob/main/README.md"
        rel="noopener"
        target="_blank"
        class="help-button"
      >
        <svg clip-rule="evenodd" fill-rule="evenodd" stroke-linejoin="round" stroke-miterlimit="2" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="m2 4v16.002c0 .385.22.735.567.902.346.166.758.119 1.058-.121l4.725-3.781h12.65c.552 0 1-.448 1-1v-12.002c0-.552-.448-1-1-1h-18c-.552 0-1 .448-1 1zm9.998 5.002c.414 0 .75.336.75.75v3.5c0 .414-.336.75-.75.75s-.75-.336-.75-.75v-3.5c0-.414.336-.75.75-.75zm.002-3c.552 0 1 .448 1 1s-.448 1-1 1-1-.448-1-1 .448-1 1-1z" fill-rule="nonzero"/></svg>
      </a>
      <div class="filler"></div>
      <h3>spin.ringrace.rs</h3>
      <p>
        { "Show off your racer!" }
        <br/>
        <button
          class="link-button"
          on:click={() => fileDialog.click()}
        >
        { "Click here" }
        </button>
        { " to load a pk3 or wad."}
      </p>
      <input
        type="file"
        accept=".pk3,.wad"
        multiple={true}
        class="hidden"
        on:change={onInputFile}
        ref={fileDialog}
      />
    </header>
  );
};

export default Header;
