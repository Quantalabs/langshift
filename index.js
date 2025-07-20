import "https://cdn.plot.ly/plotly-3.1.0-rc.0.min.js";

let { solve_abrams_strogatz, solve_diaz_switkes } = wasmBindings;

const model = document.getElementById("model").value;
document.getElementById("abrams").style.display =
	model === "abrams" ? "flex" : "none";
document.getElementById("diaz").style.display =
	model === "diaz" ? "flex" : "none";

document.getElementById("model").addEventListener("change", async (e) => {
	const model = e.target.value;
	document.getElementById("abrams").style.display =
		model === "abrams" ? "flex" : "none";
	document.getElementById("diaz").style.display =
		model === "diaz" ? "flex" : "none";

	await solve();
});

async function solve(e) {
	const model = document.getElementById("model").value;
	const params = {
		abrams: {
			x: Number(document.getElementById("x").value),
			y: Number(document.getElementById("y").value),
			c: Number(document.getElementById("c").value),
			a: Number(document.getElementById("a").value),
			s: Number(document.getElementById("s").value),
		},
		diaz: {
			x_1: Number(document.getElementById("x_1").value),
			x_2: Number(document.getElementById("x_2").value),
			b: Number(document.getElementById("b").value),
			m_1: Number(document.getElementById("m_1").value),
			m_2: Number(document.getElementById("m_2").value),
			g_1: Number(document.getElementById("g_1").value),
			g_2: Number(document.getElementById("g_2").value),
			alpha: Number(document.getElementById("alpha").value),
			beta: Number(document.getElementById("beta").value),
		},
	}[model];
	const t = document.getElementById("t").value;

	if (model === "abrams") {
		const solution = await solve_abrams_strogatz(
			params.x,
			params.y,
			params.c,
			params.a,
			params.s,
			t,
		);
		plot([
			[solution, "Monolingual 1"],
			[solution.map((y) => params.x + params.y - y), "Monolingual 2"],
		]);
	} else {
		const x = await solve_diaz_switkes(
			[params.x_1, params.x_2, params.b],
			[params.m_1, params.m_2],
			[params.g_1, params.g_2],
			params.alpha,
			params.beta,
			t,
			0,
		);
		const b = await solve_diaz_switkes(
			[params.x_1, params.x_2, params.b],
			[params.m_1, params.m_2],
			[params.g_1, params.g_2],
			params.alpha,
			params.beta,
			t,
			1,
		);
		plot([
			[x, "Monlingual 1"],
			[b, "Bilingual"],
			[
				x.map(
					(y, i) => params.x_1 + params.x_2 + params.b - (y + b[i]),
				),
				"Monlingual 2",
			],
		]);
	}
}

document.getElementById("solve").addEventListener("click", solve);

solve();

function plot(data) {
	Plotly.purge("plot");
	let input = [];
	for (let i = 0; i < data.length; i++) {
		input.push({ y: data[i][0], name: `${data[i][1]}` });
	}

	const layout = {
		title: "Lang Shift",
		xaxis: {
			title: "Time",
		},
		yaxis: {
			title: "Population",
		},
	};

	Plotly.newPlot("plot", input, layout);
}
