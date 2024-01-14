import './styles.css';
import SettingsPage from './SettingsPage.svelte';

const app = new SettingsPage({
    // @ts-ignore
    target: document.getElementById('app'),
});

export default app;
