import { Component } from "solid-js";
import "./Sidebar.scss";

export const Sidebar: Component = () => {
  return (
    <div class="sidebar">
      <nav class="menu">
        <ul>
          <li>
            <a href="#">Introduction</a>
          </li>
          <li>
            <a href="#">Setup</a>
          </li>
        </ul>
      </nav>
    </div>
  );
};
