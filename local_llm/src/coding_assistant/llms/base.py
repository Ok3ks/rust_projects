from abc import ABC, abstractmethod
from enum import Enum
from typing import Tuple

import ollama
from ollama import ChatResponse, Client, chat

class LLMProviderType(Enum):
    OLLAMA = "ollama"
    DUMMY = "dummy"


class LLM(ABC):
    """
                    Common interface for all LLMs, no matter the
    provider."""

    def __init__(self):
        self.system_prompt = None
        self.message = []

    @abstractmethod
    def generate_response(self, prompt: str) -> str:
        pass

    def set_system_prompt(self, system_prompt: str):
        """
                        Sets the initial system prompt that instructs the
        LLM to behave in a certain way."""
        self.system_prompt = system_prompt

class DummyLLM(LLM):
    """
    Dummy LLM implementation useful for quick debugging.
    """

    def __init__(self):
        # call the parent class constructor to initialize
        super().__init__()

    def generate_response(self, prompt: str) -> Tuple[str, dict[str, str]]:
        """
        Generates a dummy (and fast) response to the given prompt.
        """
        response = 'Dummy response!'

        self.messages.append(
            {
                'role': 'assistant',
                'content': response,
            }
        )

        return response, []
    
class OllamaLLM(LLM):
    """
    Ollama LLM implementation
    """

    def __init__(
            self,
            model_name: str, 
    ):
        
        super().__init__()
        self.model_name = model_name
        self.download_model_if_necessary(model_name)
        self.client = Client()

    def _download_model_if_necessary(self, model_name:str):
        """
        Downloads the `model_name` model if it is not already downloaded.
        """

        models_in_disk = [model.model for model in ollama.list().models]
        if self.model_name not in models_in_disk:
            print(f'Model {self.model_name} not found in disk, downloading....')
            ollama.pull(self.model_name)

    def generate_response(self, prompt:str) -> Tuple[str, dict[str, str]]:
        """
        Generates a response to the given prompt, given the current conversation history

        It returns the response and the updated conversation history.
        """

        if self.system_prompt is None:
            raise ValueError('Sytem prompt is not set')
        
        self.messages.append(
            {
                'role':'user',
                'content': self.system_prompt + '\n' + prompt,
            }
        )

        response: ChatResponse = chat(
            model=self.model_name,
            messages=self.messages,
            stream=False,
        )
        response: str = response.message.content

        self.messages.append({
            'role': 'assistant',
            'content': response,
        })

        return response, self.messages