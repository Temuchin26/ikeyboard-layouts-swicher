import React, { useEffect, useCallback, useState } from "react";
import cx from "classnames";
import { appWindow } from "@tauri-apps/api/window";

const LANGS: any = {
  us: "ABC",
  ru: "Russian",
  ua: "Ukrainian",
};

const initialLayouts: string[] = [];

function App() {
  const [layouts, setLayouts] = useState<string[]>(initialLayouts);
  const [currentLayout, setCurrentLayout] = useState("en");

  const onWindowOpen = useCallback(async () => {
    await appWindow.listen("layouts-change", (event: any) =>
      setLayouts(JSON.parse(event.payload.layouts))
    );

    await appWindow.listen("hide-window", (event: any) => {
      setLayouts(initialLayouts);
    });

    await appWindow.listen("current-layout-change", (event: any) =>
      setCurrentLayout(event.payload.currentLayout)
    );
  }, []);

  useEffect(() => {
    onWindowOpen();
  }, [onWindowOpen]);

  if (!layouts.length) return null;

  return (
    <div className="container">
      <div className="layouts">
        {layouts.map((layout) => (
          <div
            key={layout}
            className={cx("layout", {
              active: layout === currentLayout,
            })}
          >
            <span>{LANGS[layout]}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
