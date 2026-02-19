function initGraph() {
  const data = window.__GRAPH_DATA__;
  if (!data || !data.nodes.length) return;

  const container = document.getElementById("graph-container");
  if (!container) return;

  // Prevent double-init
  if (container.dataset.graphInit) return;
  container.dataset.graphInit = "1";

  // Read theme-aware colors from CSS variables (with fallbacks)
  const style = getComputedStyle(document.documentElement);
  const C = {
    link:           style.getPropertyValue("--graph-link").trim()                 || "#4A5568",
    linkHighlight:  style.getPropertyValue("--graph-link-highlight").trim()       || "#58A6FF",
    label:          style.getPropertyValue("--graph-label").trim()                || "#8B949E",
    nodeFill:       style.getPropertyValue("--graph-node-fill").trim()            || "#58A6FF",
    nodeStroke:     style.getPropertyValue("--graph-node-stroke").trim()          || "#161B22",
    nodeHighlight:  style.getPropertyValue("--graph-node-highlight-fill").trim()  || "#E6EDF3",
  };

  const width = container.clientWidth;
  const height = Math.min(width * 0.6, 220);
  container.style.height = height + "px";

  const isMobile = width < 500;
  const nodeRadius = isMobile ? 6 : 8;
  const linkDist = isMobile ? 60 : 100;
  const chargeStr = isMobile ? -150 : -250;
  const pad = nodeRadius + 20;

  const svg = d3
    .select(container)
    .append("svg")
    .attr("width", "100%")
    .attr("height", "100%")
    .attr("viewBox", [0, 0, width, height])
    .attr("preserveAspectRatio", "xMidYMid meet");

  const nodeMap = new Map(data.nodes.map((n) => [n.id, n]));
  const validEdges = data.edges.filter(
    (e) => nodeMap.has(e.source) && nodeMap.has(e.target)
  );

  const simulation = d3
    .forceSimulation(data.nodes)
    .force(
      "link",
      d3.forceLink(validEdges).id((d) => d.id).distance(linkDist)
    )
    .force("charge", d3.forceManyBody().strength(chargeStr))
    .force("center", d3.forceCenter(width / 2, height / 2))
    .force("collision", d3.forceCollide().radius(nodeRadius + 4))
    .force("x", d3.forceX(width / 2).strength(0.1))
    .force("y", d3.forceY(height / 2).strength(0.1))
    .stop();

  for (let i = 0; i < 300; i++) simulation.tick();

  data.nodes.forEach((d) => {
    d.x = Math.max(pad, Math.min(width - pad, d.x));
    d.y = Math.max(pad, Math.min(height - pad, d.y));
  });

  const link = svg
    .append("g")
    .selectAll("line")
    .data(validEdges)
    .join("line")
    .attr("stroke", C.link)
    .attr("stroke-width", 1.5)
    .attr("stroke-opacity", 0.8);

  const label = svg
    .append("g")
    .selectAll("text")
    .data(data.nodes)
    .join("text")
    .text((d) => d.title.length > 18 ? d.title.slice(0, 16) + "\u2026" : d.title)
    .attr("font-size", isMobile ? "8px" : "10px")
    .attr("fill", C.label)
    .attr("text-anchor", "middle")
    .attr("dy", -(nodeRadius + 4))
    .style("pointer-events", "none")
    .style("font-family", "'Anonymous Pro', monospace");

  const node = svg
    .append("g")
    .selectAll("circle")
    .data(data.nodes)
    .join("circle")
    .attr("r", nodeRadius)
    .attr("fill", C.nodeFill)
    .attr("stroke", C.nodeStroke)
    .attr("stroke-width", 1.5)
    .style("cursor", "pointer")
    .call(drag(simulation));

  const connectedNodes = new Map();
  validEdges.forEach((e) => {
    const sid = typeof e.source === "object" ? e.source.id : e.source;
    const tid = typeof e.target === "object" ? e.target.id : e.target;
    if (!connectedNodes.has(sid)) connectedNodes.set(sid, new Set());
    if (!connectedNodes.has(tid)) connectedNodes.set(tid, new Set());
    connectedNodes.get(sid).add(tid);
    connectedNodes.get(tid).add(sid);
  });

  function highlight(d) {
    const connected = connectedNodes.get(d.id) || new Set();
    node
      .attr("fill", (n) =>
        n.id === d.id || connected.has(n.id) ? C.nodeHighlight : C.nodeFill
      )
      .attr("opacity", (n) =>
        n.id === d.id || connected.has(n.id) ? 1 : 0.2
      );
    link
      .attr("stroke", (l) => {
        const sid = typeof l.source === "object" ? l.source.id : l.source;
        const tid = typeof l.target === "object" ? l.target.id : l.target;
        return sid === d.id || tid === d.id ? C.linkHighlight : C.link;
      })
      .attr("stroke-opacity", (l) => {
        const sid = typeof l.source === "object" ? l.source.id : l.source;
        const tid = typeof l.target === "object" ? l.target.id : l.target;
        return sid === d.id || tid === d.id ? 1 : 0.1;
      });
    label
      .attr("fill", (n) =>
        n.id === d.id || connected.has(n.id) ? C.nodeHighlight : C.label
      )
      .attr("opacity", (n) =>
        n.id === d.id || connected.has(n.id) ? 1 : 0.15
      );
  }

  function unhighlight() {
    node.attr("fill", C.nodeFill).attr("opacity", 1);
    link.attr("stroke", C.link).attr("stroke-opacity", 0.8);
    label.attr("fill", C.label).attr("opacity", 1);
  }

  node
    .on("mouseenter", function (event, d) { highlight(d); })
    .on("mouseleave", unhighlight)
    .on("click", function (event, d) {
      window.location.href = d.href;
    });

  function render() {
    link
      .attr("x1", (d) => d.source.x)
      .attr("y1", (d) => d.source.y)
      .attr("x2", (d) => d.target.x)
      .attr("y2", (d) => d.target.y);
    node
      .attr("cx", (d) => Math.max(pad, Math.min(width - pad, d.x)))
      .attr("cy", (d) => Math.max(pad, Math.min(height - pad, d.y)));
    label
      .attr("x", (d) => d.x)
      .attr("y", (d) => d.y);
  }
  render();

  simulation.on("tick", () => {
    data.nodes.forEach((d) => {
      d.x = Math.max(pad, Math.min(width - pad, d.x));
      d.y = Math.max(pad, Math.min(height - pad, d.y));
    });
    render();
  });

  function drag(simulation) {
    return d3
      .drag()
      .on("start", (event, d) => {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
      })
      .on("drag", (event, d) => {
        d.fx = Math.max(pad, Math.min(width - pad, event.x));
        d.fy = Math.max(pad, Math.min(height - pad, event.y));
      })
      .on("end", (event, d) => {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
      });
  }
}

// Poll until container and data are ready (handles SPA navigation)
(function waitAndInit() {
  if (window.__GRAPH_DATA__ && document.getElementById("graph-container")) {
    initGraph();
  } else {
    setTimeout(waitAndInit, 50);
  }
})();
