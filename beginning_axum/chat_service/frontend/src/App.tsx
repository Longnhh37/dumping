import { useCallback, useEffect, useState } from 'react'
import type { FormEvent } from 'react'
import './App.css'

type Room = { id: number; participants: string[] }
type Message = { sender_id: number; message: string; timestamp: string; room_id: number }
type User = { id: number; username: string }

function App() {
  const [path, setPath] = useState(window.location.pathname)
  const [username, setUsername] = useState(() => sessionStorage.getItem('username') ?? '')

  const navigate = useCallback((to: string) => {
    window.history.pushState({}, '', to)
    setPath(to)
  }, [])

  useEffect(() => {
    const onPopState = () => setPath(window.location.pathname)
    window.addEventListener('popstate', onPopState)
    return () => window.removeEventListener('popstate', onPopState)
  }, [])

  useEffect(() => {
    if (!username && path !== '/') navigate('/')
    else if (username && path === '/') navigate('/rooms')
  }, [username, path, navigate])

  const setUser = (name: string) => {
    sessionStorage.setItem('username', name)
    setUsername(name)
    navigate('/rooms')
  }

  const roomMatch = path.match(/^\/chat\/(\d+)/)

  return (
    <main className="app-shell">
      <header className="topbar">
        <button className="brand" onClick={() => navigate(username ? '/rooms' : '/')}>Axum chat app</button>
        {username && <div className="account"><span className="avatar">{username[0]?.toUpperCase()}</span><span>{username}</span><button className="text-button" onClick={() => { sessionStorage.removeItem('username'); setUsername(''); navigate('/') }}>Sign out</button></div>}
      </header>
      {!username ? <Enter onEnter={setUser} /> : roomMatch ? <Chat roomId={Number(roomMatch[1])} username={username} onBack={() => navigate('/rooms')} /> : <Rooms username={username} onJoin={(id) => navigate(`/chat/${id}`)} />}
    </main>
  )
}

function Enter({ onEnter }: { onEnter: (name: string) => void }) {
  const [name, setName] = useState('')
  const submit = (event: FormEvent) => { event.preventDefault(); if (name.trim()) onEnter(name.trim()) }
  return <section className="welcome"><div className="welcome-card"><div className="brand-mark">✦</div><p className="eyebrow">A place to talk</p><h1>Good conversations<br />start here.</h1><p className="subheading">Choose a name and join a room with your people.</p><form onSubmit={submit}><label htmlFor="username">Your display name</label><input id="username" autoFocus placeholder="e.g. Alex" value={name} onChange={(event) => setName(event.target.value)} /><button className="primary-button" type="submit">Continue <span>→</span></button></form><p className="fine-print">Your name is saved for this browser session.</p></div><div className="welcome-note"><span className="online-dot" /> Simple, open group chat</div></section>
}

function Rooms({ username, onJoin }: { username: string; onJoin: (id: number) => void }) {
  const [rooms, setRooms] = useState<Room[]>([])
  const [error, setError] = useState('')
  const loadRooms = useCallback(async () => {
    try { const response = await fetch('/api/room'); if (!response.ok) throw new Error('Could not load rooms'); setRooms(await response.json() as Room[]); setError('') }
    catch { setError('Could not connect to chat service. Please try again.') }
  }, [])
  useEffect(() => { void loadRooms() }, [loadRooms])

  const createRoom = async () => {
    try { const response = await fetch('/api/room', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ participants: [username] }) }); if (!response.ok) throw new Error(); const room = await response.json() as Room; await loadRooms(); if (room.id) onJoin(room.id) }
    catch { setError('Could not create a room. Please try again.') }
  }
  const deleteRoom = async (id: number) => {
    try { const response = await fetch(`/api/room?id=${id}`, { method: 'DELETE' }); if (!response.ok) throw new Error(); await loadRooms() }
    catch { setError('Could not delete this room. Please try again.') }
  }

  return <section className="content"><div className="page-heading"><div><p className="eyebrow">YOUR SPACE</p><h1>Chat rooms</h1><p className="subheading">Pick up a conversation or make a new space.</p></div><button className="primary-button create-button" onClick={() => void createRoom()}><span>＋</span> New room</button></div>{error && <p className="error-banner">{error}</p>}<div className="room-grid">{rooms.map((room) => <article className="room-card" key={room.id}><div className="room-card-top"><span className="room-icon">#</span><button className="delete-button" aria-label={`Delete room ${room.id}`} onClick={() => void deleteRoom(room.id)}>···</button></div><h2>Room {room.id}</h2><p className="room-meta"><span className="online-dot" />{room.participants.length} {room.participants.length === 1 ? 'member' : 'members'}</p><div className="room-card-bottom"><div className="member-stack">{room.participants.slice(0, 4).map((person, i) => <span className={`member-avatar avatar-${i % 4}`} title={person} key={`${person}-${i}`}>{person[0]?.toUpperCase()}</span>)}</div><button className="join-button" onClick={() => onJoin(room.id)}>Join room <span>→</span></button></div></article>)}</div>{!rooms.length && !error && <div className="empty-state"><div className="empty-icon">◌</div><h2>It’s quiet in here</h2><p>Create a room to start your first conversation.</p></div>}</section>
}

function Chat({ roomId, username, onBack }: { roomId: number; username: string; onBack: () => void }) {
  const [messages, setMessages] = useState<Message[]>([])
  const [users, setUsers] = useState<User[]>([])
  const [senderId, setSenderId] = useState(0)
  const [draft, setDraft] = useState('')
  const [error, setError] = useState('')
  useEffect(() => {
    let active = true
    fetch('/api/user').then((response) => response.json() as Promise<User[]>).then((data) => { if (!active) return; setUsers(data); setSenderId(data.find((user) => user.username === username)?.id ?? 0) }).catch(() => undefined)
    fetch(`/api/chat/?room_id=${roomId}`).then((response) => { if (!response.ok) throw new Error(); return response.json() as Promise<Message[]> }).then((data) => { if (active) setMessages(data) }).catch(() => { if (active) setError('Could not load message history.') })
    const events = new EventSource('/api/chat/subscribe')
    events.onmessage = (event) => { const message = JSON.parse(event.data) as Message; if (message.room_id === roomId) setMessages((current) => current.some((item) => item.timestamp === message.timestamp && item.sender_id === message.sender_id && item.message === message.message) ? current : [...current, message]) }
    events.onerror = () => setError('Live connection interrupted. Reconnecting…')
    return () => { active = false; events.close() }
  }, [roomId, username])

  const send = async (event: FormEvent) => {
    event.preventDefault(); const text = draft.trim(); if (!text) return
    try { const response = await fetch('/api/chat/send', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ id: 0, timestamp: new Date().toISOString(), sender_id: senderId, message: text, room_id: roomId }) }); if (!response.ok) throw new Error(); setDraft(''); setError('') }
    catch { setError('Message could not be sent. Please try again.') }
  }

  return <section className="chat-page"><div className="chat-heading"><button className="back-button" onClick={onBack}>← <span>All rooms</span></button><div className="chat-room-title"><span className="room-icon">#</span><div><h1>Room {roomId}</h1><p>Conversation space</p></div></div><span className="live-label"><span className="online-dot" /> Live</span></div>{error && <p className="error-banner">{error}</p>}<div className="message-list">{messages.length ? messages.map((message, index) => { const sender = users.find((user) => user.id === message.sender_id)?.username ?? `User ${message.sender_id}`; return <article className={`message ${message.sender_id === senderId ? 'mine' : ''}`} key={`${message.timestamp}-${index}`}><span className="message-avatar">{sender[0]?.toUpperCase()}</span><div className="message-body"><div className="message-meta"><strong>{message.sender_id === senderId ? 'You' : sender}</strong><time>{new Date(message.timestamp).toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' })}</time></div><p>{message.message}</p></div></article> }) : <div className="empty-state chat-empty"><div className="empty-icon">✳</div><h2>The conversation starts with you</h2><p>Send a message to get things going.</p></div>}</div><form className="composer" onSubmit={(event) => void send(event)}><input aria-label="Message" placeholder="Write a message…" value={draft} onChange={(event) => setDraft(event.target.value)} /><button className="send-button" type="submit" disabled={!draft.trim()}>Send <span>↑</span></button></form><p className="composer-hint">Chatting as <strong>{username}</strong></p></section>
}

export default App
