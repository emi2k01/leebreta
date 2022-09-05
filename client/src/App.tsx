import { Component } from "solid-js";
import "./App.scss";
import { Sidebar } from "./components/Sidebar";
import { AiOutlineColumnHeight } from "solid-icons/ai";

const App: Component = () => {
  return (
    <div id="root-wrapper">
      <Sidebar />
      <div id="content-wrapper">
        <article>
          <h1>How to build a compiler</h1>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer
            enim libero, maximus eu eros ac, tristique iaculis risus. Vivamus
            eget volutpat arcu. Integer tristique scelerisque egestas. Sed
            rutrum ligula non dolor efficitur mollis. Nulla iaculis varius
            elementum. Duis pharetra scelerisque ante, et vulputate ante iaculis
            at. Nulla vel felis ac tellus lacinia imperdiet sagittis in purus.
          </p>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer
            enim libero, maximus eu eros ac, tristique iaculis risus. Vivamus
            eget volutpat arcu. Integer tristique scelerisque egestas. Sed
            rutrum ligula non dolor efficitur mollis. Nulla iaculis varius
            elementum. Duis pharetra scelerisque ante, et vulputate ante iaculis
            at. Nulla vel felis ac tellus lacinia imperdiet sagittis in purus.
          </p>
          <div class="codeblock">
            <div class="header">
              <div class="file-info">
                <i class="file-icon devicon-html5-plain"></i>
                <span class="filename">main.rs</span>
              </div>
              <div class="actions">
                <button>
                  <AiOutlineColumnHeight />
                </button>
              </div>
            </div>
            <div class="lines-container">
              <div class="line-numbers">
                <a href="#">1</a>
                <a href="#">2</a>
                <a href="#">3</a>
                <a href="#">4</a>
                <a href="#">5</a>
              </div>
              <code class="lines">
                <div class="line">
                  <span class="code">fn main() {"{"}</span>
                </div>
                <div class="line">
                  <span class="code">fn main() {"{"}</span>
                </div>
                <div class="line">
                  <span class="code">fn main() {"{"}</span>
                </div>
              </code>
            </div>
          </div>
        </article>
      </div>
    </div>
  );
};

export default App;
