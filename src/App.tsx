import React, { useEffect, useCallback, useState } from "react";
import cx from "classnames";
import { listen } from "@tauri-apps/api/event";

const LANGS: any = {
  us: "ABC",
  ru: "Russian",
  ua: "Ukrainian",
};

function App() {
  const [layouts, setLayouts] = useState<string[]>([]);
  const [currentLayout, setCurrentLayout] = useState("en");
  const onWindowOpen = useCallback(async () => {
    await listen("layoutChanged", (event: any) => {
      setLayouts(JSON.parse(event.payload.layouts));
      setCurrentLayout(event.payload.currentLayout);
    });
  }, []);

  useEffect(() => {
    onWindowOpen();
  }, [onWindowOpen]);

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
