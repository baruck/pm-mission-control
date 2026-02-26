import { useState } from 'react'

const views = ['Journal', 'Tasks', 'Refactor', 'Settings'] as const

type View = (typeof views)[number]

export function App() {
  const [view, setView] = useState<View>('Journal')

  return (
    <div className="shell">
      <aside>
        <h1>Mission Control</h1>
        {views.map((v) => (
          <button key={v} className={view === v ? 'active' : ''} onClick={() => setView(v)}>
            {v}
          </button>
        ))}
      </aside>
      <main>
        {view === 'Journal' && <JournalView />}
        {view === 'Tasks' && <TasksView />}
        {view === 'Refactor' && <RefactorView />}
        {view === 'Settings' && <SettingsView />}
      </main>
    </div>
  )
}

function JournalView() {
  return (
    <section>
      <h2>Journal</h2>
      <p>Infinite day-grouped timeline with bucket parsing and bulk conversion entry points.</p>
      <textarea rows={16} placeholder="Type notes, buckets, and bullets here..." />
    </section>
  )
}

function TasksView() {
  return (
    <section>
      <h2>Tasks (GTD)</h2>
      <ul>
        <li>Next</li>
        <li>Waiting For</li>
        <li>Scheduled</li>
        <li>Someday</li>
        <li>Done</li>
      </ul>
    </section>
  )
}

function RefactorView() {
  return (
    <section>
      <h2>Refactor Queue</h2>
      <p>Tracks local and remote artifacts labeled need_refactor.</p>
    </section>
  )
}

function SettingsView() {
  return (
    <section>
      <h2>First-run Setup</h2>
      <button>Choose Workspace Folder…</button>
      <h3>Atlassian</h3>
      <input placeholder="https://company.atlassian.net" />
      <input placeholder="Email" />
      <input placeholder="API Token" type="password" />
      <div>
        <button>Test Jira Connection</button>
        <button>Test Confluence Connection</button>
        <button>Save</button>
        <button>Clear Credentials</button>
      </div>
      <label>
        <input type="checkbox" defaultChecked /> Treat [Topic] prefixes as tags
      </label>
      <label>
        <input type="checkbox" defaultChecked /> Auto-extract (?) lines as Questions
      </label>
    </section>
  )
}
