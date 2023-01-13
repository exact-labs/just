// esm
import { h, Component } from 'https://esm.sh/preact';
import { render } from 'https://esm.sh/preact-render-to-string';

// built in
import { server } from 'just/net';
import { fs } from 'just/io';

/** @jsx h */

class Fox extends Component {
	render({ name }) {
		return (
			<p>
				The foxes name is: <span class='fox'>{name}</span>
			</p>
		);
	}
}

const Headers = ({ styles, children }) => (
	<html>
		<style>{styles}</style>
		<body>{children}</body>
		<p>
			current location: <span id='path' />
		</p>
		<script>document.getElementById('path').innerHTML = window.location.pathname;</script>
	</html>
);

const Box = ({ type, children }) => <div class={`box box-${type}`}>{children}</div>;

let html = await fs.read.file('styles.css').then((styles) => {
	return render(
		<Headers styles={styles}>
			<Box type='open'>
				<Fox name='Fuzzy' />
			</Box>
		</Headers>
	);
});

server.string(html, 'text/html; charset=UTF-8');
