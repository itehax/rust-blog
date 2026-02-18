use crate::server_functions::posts::get_graph_data;
use leptos::*;

#[component]
pub fn GraphView(
    #[prop(into, optional)] filter_post: String,
) -> impl IntoView {
    let graph_data = create_resource(|| (), |_| async move { get_graph_data().await });
    let is_filtered = !filter_post.is_empty();
    let filter_post = store_value(filter_post);

    view! {
        <Suspense fallback=move || {
            view! { <p class="text-[#8B949E]">"Loading graph..."</p> }
        }>
            {move || {
                graph_data
                    .get()
                    .map(|result| match result {
                        Ok(data) => {
                            let fp = filter_post.get_value();
                            let filtered = if !fp.is_empty() {
                                let post_href = &fp;
                                let connected_ids: std::collections::HashSet<String> = data
                                    .edges
                                    .iter()
                                    .filter(|e| &e.source == post_href || &e.target == post_href)
                                    .flat_map(|e| vec![e.source.clone(), e.target.clone()])
                                    .collect();
                                let nodes: Vec<_> = data
                                    .nodes
                                    .iter()
                                    .filter(|n| connected_ids.contains(&n.id))
                                    .cloned()
                                    .collect();
                                let node_ids: std::collections::HashSet<String> = nodes
                                    .iter()
                                    .map(|n| n.id.clone())
                                    .collect();
                                let edges: Vec<_> = data
                                    .edges
                                    .iter()
                                    .filter(|e| {
                                        node_ids.contains(&e.source) && node_ids.contains(&e.target)
                                    })
                                    .cloned()
                                    .collect();
                                crate::server_functions::posts::GraphData {
                                    nodes,
                                    edges,
                                }
                            } else {
                                data
                            };
                            if is_filtered && filtered.nodes.len() < 2 {
                                return 
                                view! { <></> }
                                    .into_view();
                            }
                            let json = serde_json::to_string(&filtered).unwrap_or_default();
                            let graph_el = 
                            view! {
                                <div
                                    id="graph-container"
                                    class="relative w-full overflow-hidden"
                                    style="height: 250px;"
                                ></div>
                                <script>
                                    {format!(
                                        r#"window.__GRAPH_DATA__ = {};
                                        (function() {{
                                            function loadScript(src) {{
                                                var s = document.createElement('script');
                                                s.src = src;
                                                s.defer = true;
                                                document.body.appendChild(s);
                                                return new Promise(function(res, rej) {{
                                                    s.onload = res;
                                                    s.onerror = rej;
                                                }});
                                            }}
                                            if (typeof d3 !== 'undefined') {{
                                                loadScript('/js/graph.js');
                                            }} else {{
                                                loadScript('https://d3js.org/d3.v7.min.js').then(function() {{
                                                    loadScript('/js/graph.js');
                                                }});
                                            }}
                                        }})();"#,
                                        json,
                                    )}
                                </script>
                            };
                            if is_filtered {

                                view! {
                                    <div class="mt-16 border border-[#1b2029] rounded-xl p-4 bg-[#0D1117]">
                                        <h3 class="text-sm font-medium text-[#8B949E] mb-4 text-center">
                                            "Related Posts"
                                        </h3>
                                        {graph_el}
                                    </div>
                                }
                                    .into_view()
                            } else {
                                graph_el.into_view()
                            }
                        }
                        Err(e) => {
                            view! {
                                <p class="text-red-400">"Error loading graph: " {e.to_string()}</p>
                            }
                                .into_view()
                        }
                    })
            }}

        </Suspense>
    }
}
