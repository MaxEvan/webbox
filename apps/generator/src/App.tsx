import { Form } from "./components/Form";
import { SuccessView } from "./components/SuccessView";
import { useGenerateApp } from "./hooks/useGenerateApp";

function App() {
  const {
    isGenerating,
    progress,
    result,
    generate,
    reset,
    openInFinder,
    launchApp,
  } = useGenerateApp();

  return (
    <div className="max-w-lg mx-auto px-6 py-10">
      {/* Header */}
      <header className="text-center mb-10">
        <h1 className="text-3xl font-bold mb-2">WebBox</h1>
        <p className="text-gray-400">Turn any website into a native Mac app</p>
      </header>

      {/* Main Content */}
      {result?.success ? (
        <SuccessView
          appPath={result.path!}
          onOpenFinder={openInFinder}
          onLaunch={launchApp}
          onReset={reset}
        />
      ) : (
        <Form
          onSubmit={generate}
          isGenerating={isGenerating}
          progress={progress}
          error={result?.error || null}
        />
      )}
    </div>
  );
}

export default App;
