export default function HomePage() {
  return (
    <main className="flex min-h-screen w-full flex-col items-start justify-start">
      <section className="container flex w-full flex-row items-start justify-between gap-12 px-4 py-4">
        <h2 className="text-2xl font-extrabold tracking-tight">
          <span className="text-indigo-800">Stats</span> Overlay
        </h2>
        <h2 className="text-2xl font-extrabold tracking-tight">
          CPU: 0% | RAM: 0% | GPU: 0% | FPS: 0
        </h2>
      </section>
    </main>
  );
}
