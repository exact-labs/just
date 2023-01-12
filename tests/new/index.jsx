import { h, Component } from 'https://esm.sh/stable/preact@10.11.3/es2022/preact.js';
import { render } from 'https://esm.sh/v102/preact-render-to-string@5.2.6/es2022/preact-render-to-string.js';
import { server } from 'just/net';

/** @jsx h */

class Fox extends Component {
	render({ name }) {
		return <span class='fox'>{name}</span>;
	}
}

const Box = ({ type, children }) => <div class={`box box-${type}`}>{children}</div>;

let html = render(
	<Box type='open'>
		<Fox name='Finn' />
	</Box>
);

server.string(html);
